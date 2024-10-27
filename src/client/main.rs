mod sender;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;

use gtk::builders::DropTargetAsyncBuilder;
use gtk::ffi::{GtkBox, GtkDropTargetAsync};
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, Button, DropTarget, DropTargetAsync, Label};

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
// const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;

fn main() {
    sender::send_file().unwrap();
}

fn build_ui() {

}