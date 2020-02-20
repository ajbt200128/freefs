use crate::{
    datasource::{sources::s3_bucket::BucketSource, DataSource},
    inode::INode,
    inodetree::INodeTree,
};
use fuse::{
    FileType, Filesystem, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty,
    ReplyEntry, ReplyOpen, ReplyWrite, Request,
};
use libc::{ENOENT, ENOTEMPTY};
use std::ffi::OsStr;
use std::path::Path;
use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}, thread, mem};
use futures::executor::block_on;

pub struct Files {
    data_source: Arc<BucketSource>,
    inode_tree: INodeTree,
}

impl Files {
    pub fn new(data_source: BucketSource) -> Self {
        let mut new = Files {
            data_source:Arc::new(data_source),
            inode_tree: INodeTree::new(),
        };
        new.inode_tree.add(INode::new(
            "",
            1,
            0,
            UNIX_EPOCH,
            FileType::Directory,
            0,
        ));
        new.inode_tree
            .add_from_keys(new.data_source.get_keys("").unwrap(), None);
        let mut nodes = new.inode_tree.nodes.clone();
        for node in &mut nodes{
            if node.kind != FileType::Directory{
                let path = new.inode_tree.get_full_path(node.ino).unwrap();
                let (size,mtime) = new.data_source.get_data_attr(&path).unwrap_or((0,UNIX_EPOCH));
               node.size = size;
               node.mtime = mtime;
            }
        }
        new.inode_tree.nodes = nodes;
        new
    }

    fn sync_path(&self, ino:u64){
        let path = match self.inode_tree.get_full_path(ino) {
            Ok(path) => path,
            Err(e) => {
                println!("flush ERROR: {}",e);
                return
            }
        };
        let data_source = Arc::clone(&self.data_source);
        thread::spawn(move||{
            match block_on(data_source.sync_stage_area(&path)) {
                Err(e) => {
                    println!("Sync ERROR: {}",e);
                },
                _ => {},
            };
        }
        );
    }
}

impl Filesystem for Files {
    fn flush(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        _lock_owner: u64,
        reply: ReplyEmpty,
    ) {
        println!("flush {}", ino);
        self.sync_path(ino);
        reply.ok();
    }

    fn fsync(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, datasync: bool, reply: ReplyEmpty) {
        println!("fsync {},{}", ino, datasync);
        reply.ok();
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup: {:?}, {}", name.to_str(), parent);

        let entry = match self
            .inode_tree
            .get_inode_from_key_parent(name.to_str().unwrap(), parent)
        {
            Ok(entry) => entry,
            Err(e) => {
                //println!("lookup ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        reply.entry(&Duration::from_secs(1), &entry.get_file_attr(), 0);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let attr = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry.get_file_attr(),
            Err(e) => {
                println!("gettattr ERROR");
                reply.error(ENOENT);
                return;
            }
        };
        let ttl = Duration::from_secs(1);
        reply.attr(&ttl, &attr);
    }

    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        _size: Option<u64>,
        _atime: Option<SystemTime>,
        _mtime: Option<SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        println!("Set attr{:?}", ino);
        let attr = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry.get_file_attr(),
            Err(e) => {
                println!("setattr ERROR");
                reply.error(ENOENT);
                return;
            }
        };
        reply.attr(&Duration::from_secs(1), &attr);
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        reply: ReplyData,
    ) {
        let end = (offset + size as i64) as usize;
        println!("read {:?} {:?}, {}", ino, offset, size);
        let path = match self.inode_tree.get_full_path(ino) {
            Ok(path) => path,
            Err(e) => {
                println!("read ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let mut blocks = match self.data_source.get_data(&path) {
            Ok(result) => result,
            Err(e) => {
                println!("read ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        if blocks.len() < end {
            blocks.resize(end, 0);
        }
        let data = &blocks[offset as usize..end];
        reply.data(data);
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        println!("readdir: {:?}  \n offset: {:?}", ino, offset);

        let dir = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry,
            Err(e) => {
                println!("gettattr ERROR");
                reply.error(ENOENT);
                return;
            }
        };

        let mut entries = vec![dir];
        entries.append(&mut self.inode_tree.get_children(dir.ino));

        for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
            if i == 0 {
                reply.add(dir.ino, 1, FileType::Directory, &Path::new("."));
                reply.add(dir.ino, 2, FileType::Directory, &Path::new(".."));
                continue;
            }
            reply.add(entry.ino, i as i64 + 1, entry.kind, &Path::new(&entry.key));
            println!("{:?}: Adding {:?}, {:?}", i, entry.key, entry.ino);
        }
        reply.ok();
    }

    fn open(&mut self, _req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        println!("open {:?}, {:?}", ino, flags);
        reply.opened(0, flags);
    }

    fn write(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        _flags: u32,
        reply: ReplyWrite,
    ) {
        println!("Write: {:?}, {:?}, {:?} bytes", ino, offset, data.len());
        let offset = offset as usize;
        let path = match self.inode_tree.get_full_path(ino) {
            Ok(path) => path,
            Err(e) => {
                println!("write ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let mut blocks = match self.data_source.get_data(&path) {
            Ok(result) => result,
            Err(e) => {
                println!("write ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let buffer = data.len() + offset;
        if buffer > 0 {
            blocks.resize(buffer, 0);
        }
        blocks.splice(offset..(data.len() + offset), data.to_vec());
        self.data_source.put_data(&path, blocks);
        reply.written(data.len() as u32);
    }

    fn create(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        _mode: u32,
        _flags: u32,
        reply: ReplyCreate,
    ) {
        println!("Create: {:?},{:?}", name, parent);
        let name = name.to_str().unwrap();

        let parent_node = match self.inode_tree.get_inode_from_ino(parent) {
            Ok(entry) => entry,
            Err(e) => {
                println!("create ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };

        let file_ino = self.inode_tree.add_empty(
            name.to_string(),
            parent,
        );
        let full_path = self.inode_tree.get_full_path(file_ino).unwrap();
        self.data_source.put_data(&full_path, vec![]);

        let attr = match self.inode_tree.get_inode_from_ino(file_ino) {
            Ok(entry) => entry.get_file_attr(),
            Err(e) => {
                println!("create ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };

        reply.created(&Duration::from_secs(1), &attr, 0, 0, 0);
    }

    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("Unlink {:?}, {:?}", name, parent);
        let entry = match self
            .inode_tree
            .get_inode_from_key_parent(name.to_str().unwrap(), parent)
        {
            Ok(entry) => entry,
            Err(e) => {
                println!("unlink ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let full_path = match self.inode_tree.get_full_path(entry.ino) {
            Ok(path) => path,
            Err(e) => {
                println!("unlink ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        match block_on(self.data_source.delete_data(&full_path)) {
            Err(e) => {
                println!("unlink ERROR: {}",e);
            },
            _ => {}
        };
        
        reply.ok();
    }
    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        let name = name.to_str().unwrap();
        println!("mkdir {}, {}", parent, name);
       /* let parent_node = match self.inode_tree.get_inode_from_ino(parent) {
            Ok(entry) => entry,
            Err(e) => {
                println!("mkdir ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };

        let dir_ino = self.inode_tree.add_inode_dir(
            name,
            Some(parent)
        );
        let path = self.inode_tree.get_full_path(dir_ino).unwrap();
        let (res, code) =
            match self
                .bucket
                .put_object_blocking(path + "/.bzEmpty", &[], "text/plain")
            {
                Ok(result) => result,
                Err(e) => {
                    println!("mkdir ERROR: {}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
        let attr = self
            .inode_tree
            .get_inode_from_ino(dir_ino)
            .unwrap()
            .get_file_attr();
        reply.entry(&Duration::from_secs(1), &attr, 0)*/
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = name.to_str().unwrap();
        println!("rmdir {}, {}", parent, name);
        /*let dir = match self.inode_tree.get_inode_from_key_parent(name, parent) {
            Ok(entry) => entry,
            Err(e) => {
                println!("rmdir ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let children_count = self.inode_tree.get_children(dir.ino).iter().count();
        let full_path = self.inode_tree.get_full_path(dir.ino).unwrap();
        if children_count > 0 {
            reply.error(ENOTEMPTY);
        } else {
            let (result, code) = match self.bucket.delete_object_blocking(full_path + "/.bzEmpty") {
                Ok(result) => result,
                Err(e) => {
                    println!("rmdir ERROR: {}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
            assert_eq!(204, code);
            reply.ok();
        }*/
    }
}
