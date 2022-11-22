#![no_main]
#![no_std]

extern crate flipperzero_rt;

use flipperzero::println;
use flipperzero_rt::{entry, manifest};

manifest!(name = "Hello, Rust!");

entry!(main);

fn main(_args: *mut u8) -> i32 {
    println!("Hello, World");

    0
}
