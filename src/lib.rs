use s3::creds::Credentials;
use s3::region::Region;

pub mod files;
pub mod inode;
pub mod inodetree;
pub mod datasource;

pub struct Storage {
    pub name: String,
    pub region: Region,
    pub credentials: Credentials,
    pub bucket: String,
}

impl Storage {
    pub fn new(name:String, region:Region,bucket:String) -> Self{
        Self{
            name,
            region,
            credentials: Credentials::default_blocking().unwrap(),
            bucket,
        }
    }
}
