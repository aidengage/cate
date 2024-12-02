mod receiver;

use std::{env, str};
use std::net::{Ipv4Addr};
use std::path::Path;
use nix::unistd::Uid;

const ROOT_DIR: String = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir().unwrap().to_str().unwrap().to_string());
// println!("project_root: {}", project_root);
// let file_path = Path::new(&project_root).join("assets/links.txt");
// println!("file_path: {:?}", file_path);

// const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/server-upload/";
const UPLOAD_DIR: &str = "/var/www/html/files/";
// const UPLOAD_DIR: &str = "/home/cate/cate/upload/";
const PUB_ADDR: Ipv4Addr = Ipv4Addr::new(74, 130, 78, 72);
// const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
const ADDR: Ipv4Addr = Ipv4Addr::new(172, 17, 0, 2);
// docker localhost apparently ^^
// const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

const PORT: u16 = 8000;

fn main () {
    // if !Uid::effective().is_root() {
    //     panic!("You must run this executable with root permissions");
    // } else {
        receiver::receive_file();
    // }
}