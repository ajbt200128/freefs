use crate::{
    datasource::{sources::s3_bucket::BucketSource, DataSource},
    inode::INode,
    inodetree::INodeTree,
};
use fuse::{
    FileType, Filesystem, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty,
    ReplyEntry, ReplyOpen, ReplyWrite, Request,
};
use futures::executor::block_on;
use libc::{EINVAL, ENOENT, ENOTEMPTY, EROFS};
use std::ffi::OsStr;
use std::path::Path;
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};


pub struct Files {
    data_source: Arc<BucketSource>,
    inode_tree: INodeTree,
}

impl Files {
    pub fn new(data_source: BucketSource) -> Self {
        let mut new = Files {
            data_source: Arc::new(data_source),
            inode_tree: INodeTree::new(),
        };
        new.inode_tree.add(INode::new(
            "",
            1,
            0,
            UNIX_EPOCH,
            FileType::Directory,
            0,
            blake3::hash(b""),
        ));
        new.load_from_inode_tree_log();
        for mut node in &mut new.inode_tree.nodes {
            if node.kind != FileType::Directory {
                let (size, mtime) = new
                    .data_source
                    .get_data_attr(&node.hash)
                    .unwrap_or((0, UNIX_EPOCH));
                node.size = size;
                node.mtime = mtime;
            }
        }
        let user = new
            .data_source
            .get_manager_user()
            .expect("Could not find a key manager user");
        println!("User: {:?}", user);
        new.inode_tree.add_inode_dir(&user, Some(1));
        new
    }

    fn write_inode_tree(&self) {
        let log = self.inode_tree.write_all_to_string();
        self.data_source.put_log(log);
    }

    fn load_from_inode_tree_log(&mut self) {
        let nodes = self.data_source.get_log();
        self.inode_tree.add_from_keys(nodes, None);
    }

    fn sync_path(&self) {
        let data_source = Arc::clone(&self.data_source);
        let hashes = self.inode_tree.get_hash_list();
        thread::spawn(move || {
            match block_on(data_source.sync_stage_area(hashes)) {
                Err(e) => {
                    println!("Sync ERROR: {}", e);
                }
                _ => {}
            };
        });
        self.write_inode_tree();
    }

    fn folder_name_recipient(&self, name: &str) -> Option<Vec<String>> {
        if !name.contains(
            &self.data_source
                .get_manager_user()
                .expect("Could not find a key manager user"),
        ) {
            return None
        }

        let recipients: Vec<String> = keys_grpc_rs::get_users_from_string(name);
        match recipients.len() {
            0 => None,
            _ => Some(recipients),
        }
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
        self.sync_path();
        println!("{}", self.inode_tree.write_all_to_string());
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
        let hash = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry.hash,
            Err(e) => {
                println!("setattr ERROR");
                reply.error(ENOENT);
                return;
            }
        };
        let mut blocks = match self.data_source.get_data(&hash) {
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
        let hash;
        {
            let offset = offset as usize;
            let inode = match self.inode_tree.get_inode_from_ino(ino) {
                Ok(inode) => inode,
                Err(e) => {
                    println!("write ERROR: {}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
            let mut blocks = match self.data_source.get_data(&inode.hash) {
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
            let recipients = self.inode_tree.get_root_parent(ino).unwrap();
            let recipients = self.folder_name_recipient(&recipients.key);
            hash = self.data_source.put_data(blocks, recipients);
            match self.inode_tree.update_inode_hash(ino, hash.clone()) {
                Err(e) => println!("write ERROR:{}", e),
                _ => {}
            };
        }
        if let Ok((size, mtime)) = self.data_source.get_data_attr(&hash) {
            match self.inode_tree.update_inode_attr(ino, mtime, size) {
                Err(e) => println!("write ERROR:{}", e),
                _ => {}
            };
        }
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
        if parent == 1 {
            reply.error(EROFS);
            return;
        }

        let ino = self
            .inode_tree
            .add_empty(name.to_string(), blake3::hash(b""), parent);
        let recipients = self.inode_tree.get_root_parent(ino).unwrap();
        let recipients = self.folder_name_recipient(&recipients.key);
        let hash = self.data_source.put_data(vec![], recipients);
        match self.inode_tree.update_inode_hash(ino, hash) {
            Err(e) => println!("create ERROR:{}", e),
            _ => {}
        };

        let attr = match self.inode_tree.get_inode_from_ino(ino) {
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
        let ino: Option<u64>;
        {
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
            match block_on(self.data_source.delete_data(&entry.hash)) {
                Err(e) => {
                    println!("unlink ERROR: {}", e);
                }
                _ => {}
            };
            ino = Some(entry.ino);
        }
        match ino {
            Some(ino) => self.inode_tree.remove(ino),
            None => {}
        }
        println!("{}", self.inode_tree.write_all_to_string());
        self.sync_path();

        reply.ok();
    }

    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        let name = name.to_str().unwrap();
        println!("mkdir {}, {}", parent, name);
        if parent == 1 {
            if let None = self.folder_name_recipient(name) {
                reply.error(EINVAL);
                return;
            }
        }
        let dir_ino = self.inode_tree.add_inode_dir(name, Some(parent));
        let attr = self
            .inode_tree
            .get_inode_from_ino(dir_ino)
            .unwrap()
            .get_file_attr();
        reply.entry(&Duration::from_secs(1), &attr, 0);
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = name.to_str().unwrap();
        println!("rmdir {}, {}", parent, name);
        let dir = match self.inode_tree.get_inode_from_key_parent(name, parent) {
            Ok(entry) => entry.ino,
            Err(e) => {
                println!("rmdir ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let children_count = self.inode_tree.get_children(dir).iter().count();
        if children_count > 0 {
            reply.error(ENOTEMPTY);
        } else {
            self.inode_tree.remove(dir);
            reply.ok();
        }
    }
    fn init(&mut self, _req: &Request<'_>) -> Result<(), libc::c_int> {
        Ok(())
    }
    fn destroy(&mut self, _req: &Request<'_>) {}
    fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {}
    fn readlink(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyData) {
        reply.error(libc::ENOSYS);
    }
    fn mknod(
        &mut self,
        _req: &Request<'_>,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _rdev: u32,
        reply: ReplyEntry,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn symlink(
        &mut self,
        _req: &Request<'_>,
        _parent: u64,
        _name: &OsStr,
        _link: &Path,
        reply: ReplyEntry,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn rename(
        &mut self,
        _req: &Request<'_>,
        _parent: u64,
        _name: &OsStr,
        _newparent: u64,
        _newname: &OsStr,
        reply: ReplyEmpty,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn link(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _newparent: u64,
        _newname: &OsStr,
        reply: ReplyEntry,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn release(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }
    fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
        reply.opened(0, 0);
    }
    fn releasedir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }
    fn fsyncdir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: fuse::ReplyStatfs) {
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }
    fn setxattr(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _name: &OsStr,
        _value: &[u8],
        _flags: u32,
        _position: u32,
        reply: ReplyEmpty,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn getxattr(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _name: &OsStr,
        _size: u32,
        reply: fuse::ReplyXattr,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn listxattr(&mut self, _req: &Request<'_>, _ino: u64, _size: u32, reply: fuse::ReplyXattr) {
        reply.error(libc::ENOSYS);
    }
    fn removexattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, reply: ReplyEmpty) {
        reply.error(libc::ENOSYS);
    }
    fn access(&mut self, _req: &Request<'_>, _ino: u64, _mask: u32, reply: ReplyEmpty) {
        reply.error(libc::ENOSYS);
    }
    fn getlk(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: u32,
        _pid: u32,
        reply: fuse::ReplyLock,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn setlk(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: u32,
        _pid: u32,
        _sleep: bool,
        reply: ReplyEmpty,
    ) {
        reply.error(libc::ENOSYS);
    }
    fn bmap(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _blocksize: u32,
        _idx: u64,
        reply: fuse::ReplyBmap,
    ) {
        reply.error(libc::ENOSYS);
    }
}
