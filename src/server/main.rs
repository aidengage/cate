mod receiver;

use std::str;
use std::net::{Ipv4Addr};
use nix::unistd::Uid;

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/server-upload/";
// const UPLOAD_DIR: &str = "/var/www/html/files/";
// const UPLOAD_DIR: &str = "/home/cate/cate/upload/";
const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
// const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

const PORT: u16 = 8000;

fn main () {
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions");
    } else {
        receiver::receive_file();
    }
}