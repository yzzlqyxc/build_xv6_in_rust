#![no_std]
#![no_main]
mod lang_items;

use core::arch::global_asm;
use core::include_str;

global_asm!(include_str!("kernel/entry.S"));
global_asm!(include_str!("kernel/kernelvec.S"));