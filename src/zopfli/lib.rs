#![feature(plugin)]
#![plugin(flamer)]

extern crate libc;
extern crate flame;

pub mod cache;
pub mod deflate;
pub mod hash;
pub mod katajainen;
pub mod lz77;
pub mod squeeze;
pub mod symbols;
pub mod tree;
pub mod util;
pub mod zopfli;

use std::fs::File;

#[no_mangle]
pub extern fn dump_flame() {
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
