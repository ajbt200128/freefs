use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::path::Path;

use freefs::{
    datasource::sources::s3_bucket::BucketSource, files::Files, Storage,
};
use keys_grpc_rs::KeysManager;

fn main() {
    let bb = Storage {
        name: "backblaze".to_string(),
        region: Region::Custom {
            region: "us-west-002".to_string(),
            endpoint: "https://s3.us-west-002.backblazeb2.com".to_string(),
        },
        credentials: Credentials::default_blocking().unwrap(),
        bucket: "rust-test".to_string(),
    };
    let mut manager = KeysManager::new("freefs".to_string());
    //manager.auth_setup();
    manager.auth_unlock();
    let bucket = create_bucket(bb);
    let files = bucket.list_blocking("test-folder".to_string(), None).unwrap();
    for (i, (file,code)) in files.iter().enumerate(){
        assert_eq!(&200, code);
        let contents = &file.contents;
        for obj in contents{
            println!("{:?}",obj.key);
        }
    }
    let mountpoint = Path::new("/tmp/rust-test");
    let data_source = BucketSource::new(bucket,"/tmp/rust-test-transient".to_string(),"/tmp/rust-test-stage".to_string(),manager);
    let fs = Files::new(data_source);
    mount(mountpoint,fs);
}
fn mount(mountpoint: &Path, fs: Files) {
    let result = fuse::mount(fs, &mountpoint, &[]).unwrap();
    println!("{:?}", result);
}

fn create_bucket(storage: Storage) -> Bucket {
    Bucket::new(&storage.bucket, storage.region, storage.credentials).unwrap()
}
