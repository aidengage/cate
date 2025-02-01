mod receiver;

use nix::unistd::Uid;
use std::net::Ipv4Addr;
use std::str;

const UPLOAD_DIR: &str = "/var/www/html/files/";
const ADDR: Ipv4Addr = Ipv4Addr::new(172, 17, 0, 2);
// default docker localhost apparently ^^
// const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

const PORT: u16 = 8000;

fn main() {
    // checks if the program is run as root to allow the program
    // to create files and directories essential for functionallity
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions");
    } else {
        receiver::receive_file();
    }
    // this check might not be needed if it is run in a docker
    // container, although i have not tested this yet
}
