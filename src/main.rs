use s3::bucket::Bucket;
use s3::region::Region;
use std::path::Path;
use s3::creds::Credentials;

use bbfs::{files::Files,Storage};



fn main() {
    let bb = Storage{
        name: "backblaze".to_string(),
        region: Region::Custom{
            region:"us-west-002".to_string(),
            endpoint:"https://s3.us-west-002.backblazeb2.com".to_string(),
        },
        credentials: Credentials::default_blocking().unwrap(),
        bucket: "rust-test".to_string(),
    };
    let bucket = create_bucket(bb);
    let files = bucket.list_blocking("".to_string(), None).unwrap();
    for (i, (file,code)) in files.iter().enumerate(){
        assert_eq!(&200, code);
        //println!("{:?}",&file);
        let contents = &file.contents;
        for obj in contents{
            println!("{:?}",obj.key);
        }
    }
    //assert_eq!(200, code);
    //println!("{:?}",code);
    let mountpoint = Path::new("/tmp/rust-test");
    let fs = Files::new(bucket);
    mount(mountpoint,fs);

}
fn mount(mountpoint:&Path,fs:Files){
    let result = fuse::mount(fs, &mountpoint,&[]).unwrap();
    println!("{:?}",result);
}

fn create_bucket(storage:Storage) -> Bucket{
    Bucket::new(&storage.bucket, storage.region, storage.credentials).unwrap()
}
