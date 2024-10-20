mod receiver;

use std::io::{Read, Write};
use std::string::ToString;
use std::str;
use std::net::{Ipv4Addr};

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
// const UPLOAD_DIR: &str = "/home/cate/cate/upload/";
const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
// const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

const PORT: u16 = 8000;

fn main () {
    receiver::receive_file();
}