use fuse::{FileAttr, FileType};
use std::time::SystemTime;

#[derive(Clone)]
pub struct INode {
    pub key: String,
    pub ino: u64,
    pub size: u64,
    pub mtime: SystemTime,
    pub kind: FileType,
    pub parent: u64,
    data_hash:Vec<u8>,
}

impl INode {
    pub fn new(
        key: &str,
        ino: u64,
        size: u64,
        mtime: SystemTime,
        kind: FileType,
        parent: u64,
    ) -> Self {
        INode {
            key: key.to_string(),
            ino,
            size,
            mtime,
            kind,
            parent,
            data_hash:vec![],
        }
    }

    pub fn get_file_attr(&self) -> FileAttr {
        FileAttr {
            ino: self.ino,
            size: self.size,
            blocks: (self.size * 10 / 4096) / 10,
            atime: self.mtime,
            mtime: self.mtime,
            ctime: self.mtime,
            crtime: self.mtime,
            kind: self.kind,
            perm: 0o755,
            nlink: 1,
            uid: 1,
            gid: 1,
            rdev: 0,
            flags: 0,
        }
    }
}
