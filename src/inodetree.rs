use crate::{files::FileSysError, inode::INode};

pub struct INodeTree {
    nodes: Vec<INode>,
}

impl INodeTree {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn next_inode(&self) -> u64 {
        match self.nodes.last() {
            Some(entry) => entry.ino + 1,
            _ => 1,
        }
    }
    pub fn add(&mut self, inode: INode) {
        self.nodes.push(inode);
    }

    pub fn get_children(&self, ino:u64) -> Vec<&INode>{
        self.nodes.iter().filter(|e| e.parent==ino).collect()
    }

    pub fn get_inode_from_key(&self, name: &str) -> Result<&INode, FileSysError> {
        let file: Vec<&INode> = self.nodes.iter().filter(|e| e.key == name).collect();
        match file.len() {
            0 => Err(FileSysError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }

    pub fn get_inode_from_ino(&self, ino: u64) -> Result<&INode, FileSysError> {
        let file: Vec<&INode> = self.nodes.iter().filter(|e| e.ino == ino).collect();
        match file.len() {
            0 => Err(FileSysError::new("File Not Found")),
            1 => Ok(file.first().unwrap()),
            _ => panic!("Error: Duplicate keys"),
        }
    }

    pub fn get_inode_from_name_parent(
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
            .nodes
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
