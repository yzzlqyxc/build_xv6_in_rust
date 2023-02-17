#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

mod basic;
mod kernel;

use core::arch::global_asm;
use core::include_str;

global_asm!(include_str!("kernel/entry.asm"));
use crate::kernel::start::start;
use crate::kernel::start::stack0;

pub fn main() {
    println!("Hello world");
}