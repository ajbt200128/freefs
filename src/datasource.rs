use async_trait::async_trait;
use blake3;
use fuse::FileType;
use futures::future::join_all;
use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};
use walkdir::WalkDir;
use crate::keys_manager::KeysManager;

#[derive(Debug)]
pub struct DataSourceError {
    msg: String,
}

impl From<io::Error> for DataSourceError {
    fn from(error: io::Error) -> Self {
        DataSourceError {
            msg: error.to_string(),
        }
    }
}

impl DataSourceError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Error for DataSourceError {}

impl fmt::Display for DataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File error {}!", self.msg)
    }
}

pub fn hash_from_string(string: String) -> blake3::Hash {
    let mut arr: [u8; 32] = [0; 32];
    let bytes = &hex::decode(string).unwrap()[0..32];
    arr.copy_from_slice(bytes);
    blake3::Hash::from(arr)
}

#[derive(Debug)]
pub enum Area {
    Transient,
    Stage,
}

#[async_trait]
pub trait DataSource {
    fn get_area_path(&self, area: &Area) -> &String;
    fn get_keys_manager(&self) -> &KeysManager;
    fn get_data_from_source(&self, hash: &blake3::Hash) -> Result<Vec<u8>, DataSourceError>;
    async fn put_data_from_source(
        &self,
        hash: blake3::Hash,
        data: Vec<u8>,
    ) -> Result<(), DataSourceError>;
    async fn delete_data_from_source(&self, hash: &blake3::Hash) -> Result<(), DataSourceError>;
    fn get_data_attr_from_source(
        &self,
        hash: &blake3::Hash,
    ) -> Result<(u64, SystemTime), DataSourceError>;
    fn put_log_source(&self, log: String) -> Result<(), DataSourceError>;
    fn get_log_source(&self) -> Result<String, DataSourceError>;

    fn put_log(&self, log: String) {
        let absolute_path = format!("{}/.log", self.get_area_path(&Area::Stage));
        let mut file = File::create(&absolute_path).unwrap();
        match file.write_all(log.as_bytes()) {
            Ok(()) => {}
            Err(e) => {
                println!("Error writing to log {}", e);
            }
        };
    }

    fn get_log(&self) -> Vec<(blake3::Hash, String,FileType)> {
        let absolute_path = format!("{}/.log", self.get_area_path(&Area::Stage));
        let mut entries_string = String::new();
        if Path::new(&absolute_path).exists() {
            entries_string = fs::read_to_string(&absolute_path).unwrap();
        } else {
            entries_string = self.get_log_source().unwrap_or(String::new());
        }

        let entries: Vec<&str> = entries_string.split("\n").collect();
        let mut nodes: Vec<(blake3::Hash, String, FileType)> = vec![];
        for entry in entries {
            let entry: Vec<&str> = entry.split("\t").collect();
            if entry.len() == 3 {
                let kind = match entry[0] {
                    "file" => FileType::RegularFile,
                    "dir" => FileType::Directory,
                    _ => {
                        continue;
                    }
                };
                let hash = hash_from_string(entry[1].to_string());
                let key = entry[2][1..entry[2].len() - 1].to_string();
                nodes.push((hash, key, kind));
            }
        }
        nodes
    }

    fn check_in_area(&self, hash: &blake3::Hash, area: &Area) -> bool {
        let absolute_path = format!("{}/{}", self.get_area_path(area), hash.to_hex());
        Path::new(&absolute_path).exists()
    }

    fn get_data_from_area(
        &self,
        hash: &blake3::Hash,
        area: &Area,
    ) -> Result<Vec<u8>, DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), hash.to_hex());
        let buf = fs::read(&absolute_path)?;
        Ok(buf)
    }

    fn put_data_area(&self, hash: &blake3::Hash, data: &[u8], area: &Area) {
        let absolute_path = format!("{}/{}", self.get_area_path(area), hash.to_hex());
        println!("Create in area: {}", absolute_path);
        let mut file = File::create(&absolute_path).unwrap();
        match file.write_all(data) {
            Ok(()) => {}
            Err(e) => {
                println!("Error writing to area {:?}, {}", area, e);
            }
        };
    }

    fn put_hash_data_area(&self, data: &Vec<u8>, area: &Area) -> blake3::Hash {
        let hash = blake3::hash(&data);
        self.put_data_area(&hash, &data, area);
        hash
    }

    fn delete_data_area(&self, hash: &blake3::Hash, area: &Area) -> Result<(), DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), hash.to_hex());
        if Path::new(&absolute_path).exists() {
            fs::remove_file(&absolute_path)?;
        }
        Ok(())
    }

    fn get_hashes_area(&self, area: &Area) -> Result<Vec<blake3::Hash>, DataSourceError> {
        let absolute_path = format!("{}/", self.get_area_path(area));
        let dir = WalkDir::new(&absolute_path);
        let mut hashes = vec![];
        for e in dir.into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                if e.file_name() == ".log" {
                    continue;
                }
                let string = e
                    .path()
                    .to_str()
                    .unwrap()
                    .replace(&absolute_path, "")
                    .to_string();
                hashes.push(hash_from_string(string));
            }
        }
        Ok(hashes)
    }
    fn get_data_attr_area(
        &self,
        hash: &blake3::Hash,
        area: &Area,
    ) -> Result<(u64, SystemTime), DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), hash.to_hex());
        let file = fs::metadata(&absolute_path)?;
        Ok((file.len(), file.modified()?))
    }

    fn get_data(&self, hash: &blake3::Hash) -> Result<Vec<u8>, DataSourceError> {
        println!("Get data: {}", hash.to_hex());
        let data;
        if self.check_in_area(&hash, &Area::Stage) {
            data = self.get_data_from_area(&hash, &Area::Stage).unwrap();
        } else if self.check_in_area(&hash, &Area::Transient) {
            data = self.get_data_from_area(&hash, &Area::Transient).unwrap();
        } else {
            let block_data = self.get_data_from_source(&hash)?;
            self.put_data_area(&hash, &block_data, &Area::Transient);
            data = block_data;
        }
        let data = self.get_keys_manager().decrypt(data, false);
        Ok(data)
    }
    fn put_data(&self, data: Vec<u8>) -> blake3::Hash {
        let data = self.get_keys_manager().encrypt(data, None, false);
        self.put_hash_data_area(&data, &Area::Stage)
    }

    async fn delete_data(&self, hash: &blake3::Hash) -> Result<(), DataSourceError> {
        self.delete_data_area(hash, &Area::Transient)?;
        self.delete_data_area(hash, &Area::Stage)?;
        self.delete_data_from_source(hash).await?;
        Ok(())
    }

    fn get_data_attr(&self, hash: &blake3::Hash) -> Result<(u64, SystemTime), DataSourceError> {
        if self.check_in_area(&hash, &Area::Stage) {
            Ok(self.get_data_attr_area(hash, &Area::Stage)?)
        } else {
            Ok(self.get_data_attr_from_source(hash)?)
        }
    }

    async fn sync_stage_area(
        &self,
        valid_hashes: Vec<blake3::Hash>,
    ) -> Result<(), DataSourceError> {
        let mut futs = vec![];
        let hashes = self.get_hashes_area(&Area::Stage)?;
        for hash in &hashes {
            if !valid_hashes.contains(hash) {
                self.delete_data_area(hash, &Area::Stage)?;
                continue;
            }
            let data = self.get_data_from_area(&hash, &Area::Stage)?;
            println!("Syncing: {}", hash.to_hex());
            let hash = hash.clone();
            let fut = self.put_data_from_source(hash, data);
            futs.push(fut);
        }
        let log_path = format!("{}/.log", self.get_area_path(&Area::Stage));
        if Path::new(&log_path).exists() {
            let data = fs::read_to_string(&log_path).unwrap();
            self.put_log_source(data)?;
            fs::remove_file(&log_path)?;
        }
        for res in join_all(futs).await {
            res?;
        }
        for hash in hashes {
            self.delete_data_area(&hash, &Area::Stage)?;
        }
        Ok::<(), DataSourceError>(())
    }
}

pub mod sources {
    pub mod s3_bucket {
        use crate::{keys_manager::KeysManager, datasource::{Area, DataSource, DataSourceError}};
        use async_trait::async_trait;
        use s3::{bucket::Bucket, S3Error};
        use std::time::{Duration, UNIX_EPOCH};

        impl From<S3Error> for DataSourceError {
            fn from(err: S3Error) -> Self {
                Self {
                    msg: err.to_string(),
                }
            }
        }

        pub struct BucketSource {
            bucket: Bucket,
            transient_path: String,
            stage_path: String,
            keys_manager:KeysManager,
        }

        impl BucketSource {
            pub fn new(bucket: Bucket, transient_path: String, stage_path: String,keys_manager:KeysManager) -> Self {
                Self {
                    bucket,
                    transient_path,
                    stage_path,
                    keys_manager
                }
            }
        }

        #[async_trait]
        impl DataSource for BucketSource {
            fn get_area_path(&self, area: &Area) -> &String {
                match area {
                    Area::Transient => &self.transient_path,
                    Area::Stage => &self.stage_path,
                }
            }
            fn get_data_from_source(
                &self,
                hash: &blake3::Hash,
            ) -> Result<Vec<u8>, DataSourceError> {
                let (result, code) = self.bucket.get_object_blocking(hash.to_hex())?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(result)
                }
            }
            async fn put_data_from_source(
                &self,
                hash: blake3::Hash,
                data: Vec<u8>,
            ) -> Result<(), DataSourceError> {
                let (_, code) =
                    self.bucket
                        .put_object_blocking(hash.to_hex(), &data, "text/plain")?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(())
                }
            }

            async fn delete_data_from_source(
                &self,
                hash: &blake3::Hash,
            ) -> Result<(), DataSourceError> {
                let (_, code) = self.bucket.delete_object_blocking(hash.to_hex())?;
                if code != 204 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(())
                }
            }

            fn get_data_attr_from_source(
                &self,
                hash: &blake3::Hash,
            ) -> Result<(u64, std::time::SystemTime), DataSourceError> {
                let results = self
                    .bucket
                    .list_blocking(format!("{}", hash.to_hex()), None)?;
                let (result, code) = results.first().unwrap();
                if code != &200 {
                    return Err(DataSourceError::new(&format!("Error wrong code: {}", code)));
                }
                let obj = result.contents.first().unwrap();
                let time = chrono::DateTime::parse_from_rfc3339(&obj.last_modified).unwrap();
                let time = UNIX_EPOCH + Duration::from_millis(time.timestamp_millis() as u64);
                let size = obj.size;
                Ok((size, time))
            }

            fn put_log_source(&self, log: String) -> Result<(), DataSourceError> {
                let (_, code) =
                    self.bucket
                        .put_object_blocking(".log", log.as_bytes(), "text/plain")?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(())
                }
            }
            fn get_log_source(&self) -> Result<String, DataSourceError> {
                let (result, code) = self.bucket.get_object_blocking(".log")?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(String::from_utf8(result).unwrap_or("".to_string()))
                }
            }
            fn get_keys_manager(&self) -> &crate::keys_manager::KeysManager {
                &self.keys_manager
    }
        }
    }
}
