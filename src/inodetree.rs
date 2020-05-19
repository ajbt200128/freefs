use crate::inode::INode;
use fuse::FileType;
use std::{error::Error, fmt, time::UNIX_EPOCH};

#[derive(Debug)]
pub struct INodeTreeError {
    msg: String,
}

impl INodeTreeError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Error for INodeTreeError {}

impl fmt::Display for INodeTreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File error {}!", self.msg)
    }
}

pub struct INodeTree {
    pub nodes: Vec<INode>,
}

impl INodeTree {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn next_ino(&self) -> u64 {
        match self.nodes.last() {
            Some(entry) => entry.ino + 1,
            _ => 1,
        }
    }

    pub fn add(&mut self, inode: INode) {
        self.nodes.push(inode);
    }

    pub fn add_empty(&mut self, key: String, parent: u64) -> u64 {
        let ino = self.next_ino();
        self.nodes.push(INode::new(
            &key,
            ino,
            0,
            UNIX_EPOCH,
            FileType::RegularFile,
            parent,
        ));
        ino
    }

    pub fn add_from_keys(&mut self, keys: Vec<String>, parent: Option<u64>) {
        let parent = parent.unwrap_or(1);
        for mut key in keys {
            if key.contains("/") {
                let pos_of_last = key.rfind("/").unwrap();
                let folders = &key[..pos_of_last];
                let parent = self.add_inode_dir(folders, None);
                key = key[pos_of_last + 1..].to_string();
                self.add_empty(key, parent);
            } else {
                self.add_empty(key, parent);
            }
        }
    }

    pub fn add_inode_dir(&mut self, dir: &str, parent: Option<u64>) -> u64 {
        let folders = dir.split("/");
        let mut parent = parent.unwrap_or(1);
        for folder in folders {
            match self.get_inode_from_key_parent(folder, parent) {
                Ok(entry) => {
                    parent = entry.ino;
                    continue;
                }
                _ => {}
            };
            let ino = self.next_ino();

            let next_ino = ino;
            self.add(INode::new(
                folder,
                ino,
                0,
                UNIX_EPOCH,
                FileType::Directory,
                parent,
            ));
            parent = next_ino;
        }
        parent
    }

    pub fn get_children(&self, ino: u64) -> Vec<&INode> {
        self.nodes.iter().filter(|e| e.parent == ino).collect()
    }
    fn get_subtree(&self, ino: u64) -> Vec<&INode> {
        let mut entries = self.get_children(ino);
        for entry in entries.clone() {
            let mut children = self.get_subtree(entry.ino);
            entries.append(&mut children);
        }
        entries
    }

    pub fn get_full_path(&self, ino: u64) -> Result<String, INodeTreeError> {
        let node = self.get_inode_from_ino(ino)?;
        if node.ino == 1 {
            Ok("".to_string())
        } else {
            let parent_path = self.get_full_path(node.parent)?;
            if node.parent != 1 {
                Ok(format!("{}/{}", parent_path, node.key).to_owned())
            } else {
                Ok(node.key.clone())
            }
        }
    }

    pub fn get_inode_from_ino(&self, ino: u64) -> Result<&INode, INodeTreeError> {
        let file: Vec<&INode> = self.nodes.iter().filter(|e| e.ino == ino).collect();
        match file.len() {
            0 => Err(INodeTreeError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }

    pub fn get_inode_from_key_parent(
        &self,
        name: &str,
        parent_ino: u64,
    ) -> Result<&INode, INodeTreeError> {
        let file: Vec<&INode> = self
            .nodes
            .iter()
            .filter(|e| e.parent == parent_ino)
            .filter(|e| e.key == name)
            .collect();
        match file.len() {
            0 => Err(INodeTreeError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }
}
