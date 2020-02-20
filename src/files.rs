use crate::{inode::INode, inodetree::INodeTree};
use chrono::DateTime;
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty,
    ReplyEntry, ReplyOpen, ReplyWrite, Request,
};
use libc::{ENOENT, ENOSYS, ENOTEMPTY};
use s3::bucket::Bucket;
use s3::serde_types::Object;
use std::ffi::OsStr;
use std::path::Path;
use std::{
    error::Error,
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct FileSysError {
    msg: String,
}

impl FileSysError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Error for FileSysError {}

impl fmt::Display for FileSysError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File error {}!", self.msg)
    }
}

pub struct Files {
    bucket: Bucket,
    inode_tree: INodeTree,
}

impl Files {
    pub fn new(bucket: Bucket) -> Self {
        let mut new = Files {
            bucket,
            inode_tree: INodeTree::new(),
        };
        new.add_inode_table_entry("".to_string(), 0, 0, UNIX_EPOCH, FileType::Directory);
        new.load_dir_inode_table("");
        new
    }

    fn add_inode_table_entry_obj(&mut self, file: Object, parent: u64) -> u64 {
        let mtime = DateTime::parse_from_rfc3339(&file.last_modified).unwrap();
        let mtime = UNIX_EPOCH + Duration::from_millis(mtime.timestamp_millis() as u64);
        let next_ino =
            self.add_inode_table_entry(file.key, parent, file.size, mtime, FileType::RegularFile);
        next_ino
    }

    fn add_inode_table_entry(
        &mut self,
        key: String,
        parent: u64,
        size: u64,
        mtime: SystemTime,
        kind: FileType,
    ) -> u64 {
        let next_ino = self.inode_tree.next_inode();
        let entry = INode::new(&key, next_ino, size, mtime, kind, parent);
        self.inode_tree.add(entry);
        next_ino
    }

    fn add_inode_table_entry_dir(&mut self, dir: &str) -> u64 {
        let folders = dir.split("/");
        let mut curr_path = "".to_string();
        let mut parent = 1;
        for folder in folders {
            let key = curr_path.clone() + folder;
            curr_path = curr_path + folder + "/";
            match self.inode_tree.get_inode_from_key(&key) {
                Ok(entry) => {
                    parent = entry.ino;
                    continue;
                }
                _ => {}
            };

            let next_ino =
                self.add_inode_table_entry(key, parent, 0, UNIX_EPOCH, FileType::Directory);
            parent = next_ino;
        }
        parent
    }

    fn load_dir_inode_table(&mut self, dir: &str) {
        let files = self.bucket.list_blocking(dir.to_string(), None).unwrap();
        for (list, code) in files {
            assert_eq!(200, code);
            for file in list.contents {
                let mut parent = 1;
                if file.key.contains("/") {
                    let pos_of_last = file.key.rfind("/").unwrap();
                    let folders = &file.key[..pos_of_last];
                    parent = self.add_inode_table_entry_dir(folders);
                }
                if !file.key.contains(".bzEmpty") {
                    self.add_inode_table_entry_obj(file, parent);
                }
            }
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
        reply.ok();
    }

    fn fsync(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, datasync: bool, reply: ReplyEmpty) {
        println!("fsync {},{}", ino, datasync);
        reply.ok();
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup: {:?}, {}", name.to_str(), parent);
        let attr = match self
            .inode_tree
            .get_inode_from_name_parent(name.to_str().unwrap(), parent)
        {
            Ok(entry) => entry.getFileAttr(),
            Err(e) => {
                //println!("lookup ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        reply.entry(&Duration::from_secs(1), &attr, 0);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let attr = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry.getFileAttr(),
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
            Ok(entry) => entry.getFileAttr(),
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
        let file_key = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => &entry.key,
            Err(e) => {
                println!("read ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let (mut blocks, code) = match self.bucket.get_object_blocking(&file_key) {
            Ok(result) => result,
            Err(e) => {
                println!("read ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        assert_eq!(code, 200);
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

        let entries = self.inode_tree.get_children(dir.ino);

        for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
            if i == 0 && dir.ino == 1 {
                reply.add(1, 1, FileType::Directory, &Path::new("."));
                reply.add(1, 2, FileType::Directory, &Path::new(".."));
                continue;
            }
            reply.add(
                entry.ino,
                i as i64 + 1,
                entry.kind,
                &Path::new(&entry.key.replace(&format!("{}{}", dir.key, "/"), "")),
            );
            println!(
                "{:?}: Adding {:?}, {:?}",
                i,
                entry.key,
                entry.getFileAttr().ino
            );
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
        let file = match self.inode_tree.get_inode_from_ino(ino) {
            Ok(entry) => entry,
            Err(e) => {
                println!("write ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let (mut blocks, code) = match self.bucket.get_object_blocking(&file.key) {
            Ok(result) => result,
            Err(e) => {
                println!("write ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        assert_eq!(code, 200);
        let buffer = data.len() + offset;
        if buffer > 0 {
            blocks.resize(buffer, 0);
        }
        blocks.splice(offset..(data.len() + offset), data.to_vec());
        let (_, code) =
            match self
                .bucket
                .put_object_blocking(&file.key, blocks.as_slice(), "text/plain")
            {
                Ok(result) => result,
                Err(e) => {
                    println!("write ERROR:{}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
        assert_eq!(code, 200);
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

        let parent_name = match self.inode_tree.get_inode_from_ino(parent) {
            Ok(entry) => entry.key.clone(),
            Err(e) => {
                println!("create ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let parent_name = if parent_name != "".to_string() {
            format!("{}/", parent_name)
        } else {
            parent_name
        };
        let key = format!("{}{}", parent_name, name);
        let (_, code) = match self.bucket.put_object_blocking(&key, &[], "text/plain") {
            Ok(result) => result,
            Err(e) => {
                println!("create ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        assert_eq!(code, 200);

        let file_ino =
            self.add_inode_table_entry(key, parent, 0, UNIX_EPOCH, FileType::RegularFile);
        let attr = match self.inode_tree.get_inode_from_ino(file_ino) {
            Ok(entry) => entry.getFileAttr(),
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
        let key = match self
            .inode_tree
            .get_inode_from_name_parent(name.to_str().unwrap(), parent)
        {
            Ok(entry) => &entry.key,
            Err(e) => {
                println!("unlink ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let (res, code) = match self.bucket.delete_object_blocking(key) {
            Ok(result) => result,
            Err(e) => {
                println!("unlink ERROR:{}", e);
                reply.error(ENOENT);
                return;
            }
        };
        assert_eq!(204, code);
        reply.ok();
    }
    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        let name = name.to_str().unwrap();
        println!("mkdir {}, {}", parent, name);
        let parent_name = match self.inode_tree.get_inode_from_ino(parent) {
            Ok(entry) => entry.key.clone(),
            Err(e) => {
                println!("mkdir ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let parent_name = if parent_name != "" {
            format!("{}/", parent_name)
        } else {
            parent_name
        };
        let dir = parent_name + name;
        let empty_placeholder = format!("{}/.bzEmpty", &dir);
        let (res, code) =
            match self
                .bucket
                .put_object_blocking(&empty_placeholder, &[], "text/plain")
            {
                Ok(result) => result,
                Err(e) => {
                    println!("mkdir ERROR: {}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
        let dir_ino = self.add_inode_table_entry_dir(&dir);
        /*self.add_inode_table_entry(
            empty_placeholder,
            dir_ino,
            0,
            UNIX_EPOCH,
            FileType::RegularFile,
        );*/
        let attr = self
            .inode_tree
            .get_inode_from_ino(dir_ino)
            .unwrap()
            .getFileAttr();
        reply.entry(&Duration::from_secs(1), &attr, 0)
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = name.to_str().unwrap();
        println!("rmdir {}, {}", parent, name);
        let dir = match self.inode_tree.get_inode_from_name_parent(name, parent) {
            Ok(entry) => entry,
            Err(e) => {
                println!("rmdir ERROR: {}", e);
                reply.error(ENOENT);
                return;
            }
        };
        let children_count = self.inode_tree.get_children(dir.ino).iter().count();
        if children_count > 0 {
            reply.error(ENOTEMPTY);
        } else {
            let (result, code) = match self
                .bucket
                .delete_object_blocking(format!("{}/.bzEmpty", &dir.key))
            {
                Ok(result) => result,
                Err(e) => {
                    println!("rmdir ERROR: {}", e);
                    reply.error(ENOENT);
                    return;
                }
            };
            assert_eq!(204, code);
            reply.ok();
        }
    }
}
