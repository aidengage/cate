mod receiver;

use std::{env, str};
use std::net::{Ipv4Addr};
use std::path::Path;
use nix::unistd::Uid;

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