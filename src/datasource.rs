use async_trait::async_trait;
use blake2::{Blake2b, Digest};
use futures::future::join_all;
use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::prelude::*,
    io::{self, Read},
    path::Path,
    time::SystemTime,
};
use walkdir::WalkDir;

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

#[derive(Debug)]
pub enum Area {
    Transient,
    Stage,
}

#[async_trait]
pub trait DataSource {
    fn get_area_path(&self, area: &Area) -> &String;
    fn get_data_from_source(&self, path: &str) -> Result<Vec<u8>, DataSourceError>;
    async fn put_data_from_source(&self, path: &str, data: Vec<u8>) -> Result<(), DataSourceError>;
    async fn delete_data_from_source(&self, path: &str) -> Result<(), DataSourceError>;
    fn get_keys_from_source(&self, path: &str) -> Result<Vec<String>, DataSourceError>;
    fn get_data_attr_from_source(&self, path: &str) -> Result<(u64, SystemTime), DataSourceError>;
    //fn get_dir_hash_from_source(&self, path: &str) -> Result<Vec<u8>, DataSourceError>;
    //fn get_data_hash_from_source(&self, path: &str) -> Result<Vec<u8>, DataSourceError>;

    fn check_in_area(&self, path: &str, area: &Area) -> Result<Vec<u8>, DataSourceError> {
        let mut buf = self.get_data_from_area(path, area)?;
        let mut hasher = Blake2b::new();
        hasher.input(&mut buf);
        let res = hasher.result();
        Ok(res.to_vec())
    }

    fn get_data_from_area(&self, path: &str, area: &Area) -> Result<Vec<u8>, DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), path);
        let buf = fs::read(&absolute_path)?;
        Ok(buf)
    }

    fn put_data_area(&self, path: &str, data: &Vec<u8>, area: &Area) {
        let absolute_path = format!("{}/{}", self.get_area_path(area), path);
        let dir = Path::new(&absolute_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        fs::create_dir_all(dir).unwrap();
        let mut file = File::create(&absolute_path).unwrap();
        match file.write_all(data) {
            Ok(()) => {}
            Err(e) => {
                println!("Error writing to area {:?}, {}", area, e);
            }
        };
    }

    fn delete_data_area(&self, path: &str, area: &Area) -> Result<(), DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), path);
        if Path::new(&absolute_path).exists() {
            fs::remove_file(&absolute_path)?;
        }
        Ok(())
    }

    fn get_keys_area(&self, path: &str, area: &Area) -> Result<Vec<String>, DataSourceError> {
        let path_beginning = format!("{}/", self.get_area_path(area));
        let absolute_path = format!("{}/{}", self.get_area_path(area), path);
        let dir = WalkDir::new(&absolute_path);
        let mut keys = vec![];
        for e in dir.into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                keys.push(
                    e.path()
                        .to_str()
                        .unwrap()
                        .replace(&path_beginning, "")
                        .to_string(),
                );
            }
        }
        Ok(keys)
    }
    fn get_data_attr_area(
        &self,
        path: &str,
        area: &Area,
    ) -> Result<(u64, SystemTime), DataSourceError> {
        let absolute_path = format!("{}/{}", self.get_area_path(area), path);
        let file = fs::metadata(&absolute_path)?;
        Ok((file.len(), file.modified()?))
    }

    fn get_data(&self, path: &str) -> Result<Vec<u8>, DataSourceError> {
        println!("Get data: {}", path);
        if let Ok(_) = self.check_in_area(path, &Area::Transient) {
            Ok(self.get_data_from_area(path, &Area::Transient)?)
        } else {
            if let Ok(_) = self.check_in_area(path, &Area::Stage) {
                Ok(self.get_data_from_area(path, &Area::Stage)?)
            } else {
                let data = self.get_data_from_source(path)?;
                self.put_data_area(path, &data, &Area::Transient);
                Ok(data)
            }
        }
    }
    fn put_data(&self, path: &str, data: Vec<u8>) {
        self.put_data_area(path, &data, &Area::Stage);
    }
    async fn delete_data(&self, path: &str) -> Result<(), DataSourceError> {
        self.delete_data_area(path, &Area::Transient)?;
        self.delete_data_area(path, &Area::Stage)?;
        self.delete_data_from_source(path).await?;
        Ok(())
    }

    fn get_keys(&self, path: &str) -> Result<Vec<String>, DataSourceError> {
        let mut stage_keys = self.get_keys_area(path, &Area::Stage)?;
        let mut source_keys = self.get_keys_from_source(path)?;
        stage_keys.append(&mut source_keys);
        stage_keys.dedup();
        println!("keys {:?}", stage_keys);
        Ok(stage_keys)
    }

    fn get_data_attr(&self, path: &str) -> Result<(u64, SystemTime), DataSourceError> {
        if let Ok(_) = self.check_in_area(path, &Area::Stage) {
            Ok(self.get_data_attr_area(path, &Area::Stage)?)
        } else {
            Ok(self.get_data_attr_from_source(path)?)
        }
    }
    //fn get_dir_hash(&self, path: &str) -> Result<Vec<u8>, DataSourceError>;
    //fn get_data_hash(&self, path: &str) -> Result<Vec<u8>, DataSourceError>;
    async fn sync_stage_area(&self, path: &str) -> Result<(), DataSourceError> {
        let keys = self.get_keys_area(path, &Area::Stage)?;
        let mut futs = vec![];
        for key in &keys {
            let abs_path = format!("{}/{}", self.get_area_path(&Area::Stage), key);
            if Path::new(&abs_path).is_dir() {
                continue;
            }
            let data = self.get_data_from_area(&key, &Area::Stage)?;
            println!("Syncing: {}", key);
            let fut = self.put_data_from_source(&key, data);
            futs.push(fut);
        }
        for res in join_all(futs).await {
            res?;
        }
        for key in keys {
            self.delete_data_area(&key, &Area::Stage)?;
        }
        Ok::<(), DataSourceError>(())
    }
}

pub mod sources {
    pub mod s3_bucket {
        use crate::datasource::{Area, DataSource, DataSourceError};
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
        }

        impl BucketSource {
            pub fn new(bucket: Bucket, transient_path: String, stage_path: String) -> Self {
                Self {
                    bucket,
                    transient_path,
                    stage_path,
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
            fn get_data_from_source(&self, path: &str) -> Result<Vec<u8>, DataSourceError> {
                let (result, code) = self.bucket.get_object_blocking(path.to_string())?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(result)
                }
            }
            async fn put_data_from_source(
                &self,
                path: &str,
                data: Vec<u8>,
            ) -> Result<(), DataSourceError> {
                let (_, code) =
                    self.bucket
                        .put_object_blocking(path.to_string(), &data, "text/plain")?;
                if code != 200 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(())
                }
            }

            async fn delete_data_from_source(&self, path: &str) -> Result<(), DataSourceError> {
                let (_, code) = self.bucket.delete_object_blocking(path.to_string())?;
                if code != 204 {
                    Err(DataSourceError::new(&format!("Error wrong code: {}", code)))
                } else {
                    Ok(())
                }
            }
            fn get_keys_from_source(&self, path: &str) -> Result<Vec<String>, DataSourceError> {
                let results = self.bucket.list_blocking(path.to_string(), None)?;
                let mut keys = vec![];
                for (result, code) in results {
                    if code != 200 {
                        return Err(DataSourceError::new(&format!("Error wrong code: {}", code)));
                    } else {
                        for obj in result.contents {
                            if !obj.key.contains(".bzEmpty") {
                                keys.push(obj.key);
                            }
                        }
                    }
                }
                Ok(keys)
            }
            fn get_data_attr_from_source(
                &self,
                path: &str,
            ) -> Result<(u64, std::time::SystemTime), DataSourceError> {
                let results = self.bucket.list_blocking(path.to_string(), None)?;
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
        }
    }
}
