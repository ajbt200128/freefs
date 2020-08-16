use crate::inode::INode;
use fuse::FileType;
use std::{
    error::Error,
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

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

    pub fn remove(&mut self, ino: u64) {
        if let Some(pos) = self.nodes.iter().position(|e| e.ino == ino) {
            self.nodes.remove(pos);
        }
    }

    pub fn add(&mut self, inode: INode) {
        self.nodes.push(inode);
    }

    pub fn add_empty(&mut self, key: String, hash: blake3::Hash, parent: u64) -> u64 {
        let ino = self.next_ino();
        self.nodes.push(INode::new(
            &key,
            ino,
            0,
            UNIX_EPOCH,
            FileType::RegularFile,
            parent,
            hash,
        ));
        ino
    }

    pub fn add_from_keys(
        &mut self,
        nodes: Vec<(blake3::Hash, String, FileType)>,
        parent: Option<u64>,
    ) {
        let mut parent = parent.unwrap_or(1);
        for mut node in nodes {
            if node.2 == FileType::Directory {
                self.add_inode_dir(&node.1, None);
                continue;
            }
            if node.1.contains("/") {
                let pos_of_last = node.1.rfind("/").unwrap();
                let folders = &node.1[..pos_of_last];
                parent = self.add_inode_dir(folders, None);
                node.1 = node.1[pos_of_last + 1..].to_string();
            }
            self.add_empty(node.1, node.0, parent);
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
                SystemTime::now(),
                FileType::Directory,
                parent,
                blake3::hash(b""),
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

    pub fn update_inode_hash(
        &mut self,
        ino: u64,
        hash: blake3::Hash,
    ) -> Result<(), INodeTreeError> {
        if let Some(mut inode) = self
            .nodes
            .iter_mut()
            .filter(|e| e.ino == ino)
            .collect::<Vec<&mut INode>>()
            .first_mut()
        {
            inode.hash = hash;
            Ok(())
        } else {
            Err(INodeTreeError::new("File Not Found"))
        }
    }

    pub fn update_inode_attr(
        &mut self,
        ino: u64,
        mtime: SystemTime,
        size: u64,
    ) -> Result<(), INodeTreeError> {
        if let Some(mut inode) = self
            .nodes
            .iter_mut()
            .filter(|e| e.ino == ino)
            .collect::<Vec<&mut INode>>()
            .first_mut()
        {
            inode.mtime = mtime;
            inode.size = size;
            Ok(())
        } else {
            Err(INodeTreeError::new("File Not Found"))
        }
    }

    pub fn write_all_to_string(&self) -> String {
        let mut final_string = String::new();
        for node in &self.nodes {
            let kind = match node.kind {
                FileType::Directory => "dir".to_string(),
                FileType::RegularFile => "file".to_string(),
                _ => {
                    continue;
                }
            };
            let key = format!("\"{}\"", self.get_full_path(node.ino).unwrap());
            let hash = node.hash.to_hex();
            let string = format!("{}\t{}\t{}\n", kind, hash, key);
            final_string = final_string + &string;
        }
        final_string
    }

    pub fn get_hash_list(&self) -> Vec<blake3::Hash> {
        let mut hashes = vec![];
        for node in &self.nodes {
            if node.kind == FileType::Directory {
                continue;
            }
            hashes.push(node.hash.clone());
        }
        hashes
    }

    pub fn get_root_parent(&self, ino:u64) -> Result<&INode,INodeTreeError>{
        let node = self.get_inode_from_ino(ino)?;
        match node.parent{
            1 => Ok(node),
            _ => Ok(self.get_root_parent(node.parent)?)
        }
    }
}
