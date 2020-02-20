use chrono::DateTime;
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty,
    ReplyEntry, ReplyOpen, ReplyWrite, Request,
};
use libc::{ENOENT, ENOSYS};
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
struct FileSysError {
    msg: String,
}

impl FileSysError {
    fn new(msg: &str) -> Self {
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

struct INode {
    key: String,
    attr: FileAttr,
    parent: u64,
}

pub struct Files {
    bucket: Bucket,
    inode_table: Vec<INode>,
}

impl Files {
    pub fn new(bucket: Bucket) -> Self {
        let mut new = Files {
            bucket,
            inode_table: vec![],
        };
        new.add_inode_table_entry("".to_string(), 0, 0, UNIX_EPOCH, FileType::Directory);
        new.load_dir_inode_table("");
        new
    }

    fn next_inode(&self) -> u64 {
        match self.inode_table.last() {
            Some(entry) => entry.attr.ino + 1,
            _ => 1,
        }
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
        let next_ino = self.next_inode();
        let entry = INode {
            key,
            attr: FileAttr {
                ino: next_ino,
                size,
                blocks: (size * 10 / 4096) / 10,
                atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                mtime,
                ctime: UNIX_EPOCH,
                crtime: UNIX_EPOCH,
                kind,
                perm: 0o755,
                nlink: 0,
                uid: 501,
                gid: 20,
                rdev: 0,
                flags: 0,
            },
            parent,
        };
        self.inode_table.push(entry);
        next_ino
    }

    fn add_inode_table_entry_dir(&mut self, dir: &str) -> u64 {
        let folders = dir.split("/");
        let mut curr_path = "".to_string();
        let mut parent = 1;
        for folder in folders {
            let key = curr_path.clone() + folder;
            match self.get_inode_from_key(&key) {
                Ok(entry) => {
                    parent = entry.attr.ino;
                    continue;
                }
                _ => {}
            };
            let next_ino =
                self.add_inode_table_entry(key, parent, 0, UNIX_EPOCH, FileType::Directory);
            parent = next_ino;
            curr_path = curr_path + folder + "/";
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

                self.add_inode_table_entry_obj(file, parent);
            }
        }
    }

    fn get_inode_from_key(&self, name: &str) -> Result<&INode, FileSysError> {
        let file: Vec<&INode> = self.inode_table.iter().filter(|e| e.key == name).collect();
        match file.len() {
            0 => Err(FileSysError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }

    fn get_inode_from_ino(&self, ino: u64) -> Result<&INode, FileSysError> {
        let file: Vec<&INode> = self
            .inode_table
            .iter()
            .filter(|e| e.attr.ino == ino)
            .collect();
        match file.len() {
            0 => Err(FileSysError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }

    fn get_inode_from_name_parent(
        &self,
        name: &str,
        parent_ino: u64,
    ) -> Result<&INode, FileSysError> {
        let mut parent_name = self.get_inode_from_ino(parent_ino)?.key.clone();
        parent_name = if parent_name != "" {
            format!("{}/", parent_name)
        } else {
            parent_name
        };
        let file: Vec<&INode> = self
            .inode_table
            .iter()
            .filter(|e| e.parent == parent_ino)
            .filter(|e| e.key == format!("{}{}", parent_name, &name))
            .collect();
        match file.len() {
            0 => Err(FileSysError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }
}

impl Filesystem for Files {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup: {:?}", name.to_str());
        let attr =
            match self.get_inode_from_name_parent(name.to_str().unwrap(), parent) {
                Ok(entry) => &entry.attr,
                Err(e) => {
                    reply.error(ENOENT);
                    return;
                }
            };
        reply.entry(&Duration::from_secs(1), attr, 0);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let attr = match self.get_inode_from_ino(ino) {
            Ok(entry) => &entry.attr,
            Err(e) => {
                println!("gettattr ERROR");
                reply.error(ENOSYS);
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
        let attr = match self.get_inode_from_ino(ino){
            Ok(entry) => entry.attr,
            Err(e) => {
                println!("setattr ERROR");
                reply.error(ENOSYS);
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
        _size: u32,
        reply: ReplyData,
    ) {
        println!("read {:?} {:?}", ino, offset);
        let file_key = match self.get_inode_from_ino(ino) {
            Ok(entry) => &entry.key,
            Err(e) => {
                println!("read ERROR");
                reply.error(ENOSYS);
                return;
            }
        };
        let (blocks, code) = match self.bucket.get_object_blocking(&file_key){
            Ok(result) => result,
            Err(e) => {
                println!("Read error!");
                reply.error(ENOSYS);
                return;
            }
        };
        assert_eq!(code, 200);
        let data = &blocks[offset as usize..];
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

        let dir = match self.get_inode_from_ino(ino) {
            Ok(entry) => entry,
            Err(e) => {
                println!("gettattr ERROR");
                reply.error(ENOSYS);
                return;
            }
        };

        let entries = self.inode_table.iter().filter(|e| e.parent == ino);

        for (i, entry) in entries.enumerate().skip(offset as usize) {
            if i == 0 && ino == 1 {
                reply.add(1, 1, FileType::Directory, &Path::new("."));
                reply.add(1, 2, FileType::Directory, &Path::new(".."));
                continue;
            }
            reply.add(
                entry.attr.ino,
                i as i64 + 1,
                entry.attr.kind,
                &Path::new(&entry.key.replace(&format!("{}{}", dir.key, "/"), "")),
            );
            println!("{:?}: Adding {:?}, {:?}", i, entry.key, entry.attr.ino);
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
        let file = match self.get_inode_from_ino(ino) {
            Ok(entry) => entry,
            Err(e) => {
                println!("write ERROR: {}",e);
                reply.error(ENOSYS);
                return;
            }
        };
        let (mut blocks, code) = match self.bucket.get_object_blocking(&file.key){
            Ok(result) => result,
            Err(e) => {
                println!("write ERROR:{}",e);
                reply.error(ENOSYS);
                return;
            }
        };
        assert_eq!(code, 200);
        let buffer = data.len() + offset;
        if buffer > 0 {
            blocks.resize(buffer, 0);
        }
        blocks.splice(offset..(data.len() + offset), data.to_vec());
        let (_, code) = match self
            .bucket
            .put_object_blocking(&file.key, blocks.as_slice(), "text/plain") {
                Ok(result) => result,
                Err(e) => {
                    println!("write ERROR:{}",e);
                    reply.error(ENOSYS);
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

        let parent_name = match self.get_inode_from_ino(parent) {
            Ok(entry) => entry.key.clone(),
            Err(e) => {
                println!("create ERROR:{}",e);
                reply.error(ENOSYS);
                return;
            }
        };
        let parent_name = if parent_name != "".to_string() {
            format!("{}/", parent_name)
        } else {
            parent_name
        };
        let key = format!("{}{}", parent_name, name);
        let (_, code) = match self
            .bucket
            .put_object_blocking(&key, &[], "text/plain") {
                Ok(result) => result,
                Err(e) => {
                    println!("create ERROR:{}",e);
                    reply.error(ENOSYS);
                    return;
                }
            };
        assert_eq!(code, 200);

        let file_ino =
            self.add_inode_table_entry(key, parent, 0, UNIX_EPOCH, FileType::RegularFile);
        let attr = match self.get_inode_from_ino(file_ino) {
            Ok(entry) => entry.attr,
            Err(e) => {
                println!("create ERROR:{}",e);
                reply.error(ENOSYS);
                return;
            }
        };

        reply.created(&Duration::from_secs(1), &attr, 0, 0, 0);
    }

    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("Unlink {:?}, {:?}", name, parent);
        let key = match self
            .get_inode_from_name_parent(name.to_str().unwrap(), parent) {
                Ok(entry) => &entry.key,
                Err(e) => {
                    println!("unlink ERROR:{}",e);
                    reply.error(ENOSYS);
                    return;
                }
            };
        let (res, code) = match self.bucket.delete_object_blocking(key) {
            Ok(result) => result,
            Err(e) => {
                println!("unlink ERROR:{}",e);
                reply.error(ENOSYS);
                return;
            }
        };
        assert_eq!(204, code);
        reply.ok();
    }
}
