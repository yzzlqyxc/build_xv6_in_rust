#![no_std]
#![no_main]
#![allow(unused)]
mod lang_items;
pub mod kernel;

use core::arch::global_asm;
use core::include_str;

const TRAPFRAME : i64 = 123;

use kernel::*;

global_asm!(include_str!("kernel/assemble/entry.S"));
global_asm!(include_str!("kernel/assemble/kernelvec.S"));
global_asm!(include_str!("kernel/assemble/swtch.S"));
global_asm!(include_str!("kernel/assemble/trampoline.S"));
