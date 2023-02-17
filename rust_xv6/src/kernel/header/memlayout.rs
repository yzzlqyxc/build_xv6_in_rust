// Physical memory layout

// qemu -machine virt is set up like this,
// based on qemu's hw/riscv/virt.c:
//
// 00001000 -- boot ROM, provided by qemu
// 02000000 -- CLINT
// 0C000000 -- PLIC
// 10000000 -- uart0 
// 10001000 -- virtio disk 
// 80000000 -- boot ROM jumps here in machine mode
//             -kernel loads the kernel here
// unused RAM after 80000000.

// the kernel uses physical memory thus:
// 80000000 -- entry.S, then kernel text and data
// end -- start of kernel page allocation area
// PHYSTOP -- end RAM used by the kernel

// qemu puts UART registers here in physical memory.

pub static UART0 : u64 = 0x10000000;
pub static UART0_IRQ : u64 = 10;

// virtio mmio interface
pub static VIRTIO0 : u64 = 0x10001000;
pub static VIRTIO0_IRQ : u64 = 1;

// core local interruptor (CLINT), which contains the timer.
pub static CLINT : u64 = 0x2000000;
fn CLINT_MTIMECMP(hartid : u64) -> u64 {
    CLINT + 0x4000 + 8*(hartid)
}
fn CLINT_MTIME( clint : u64) -> u64 {
    clint + 0xBFF8 // cycles since boot.
} 

// qemu puts platform-level interrupt controller (PLIC) here.
pub static PLIC : u64 = 0x0c000000;
pub static PLIC_PRIORITY : u64 = PLIC + 0x0;
fn PLIC_PENDING(plic : u64) -> u64 {
    plic + 0x1000
}

fn PLIC_MENABLE(hart : u64) -> u64 {
    PLIC + 0x2000 + (hart)*0x100
}
fn PLIC_SENABLE(hart : u64) -> u64 {
    PLIC + 0x2080 + (hart)*0x100
}
fn PLIC_MPRIORITY(hart : u64) -> u64 {
    PLIC + 0x200000 + (hart)*0x2000
}
fn PLIC_SPRIORITY(hart : u64) -> u64 {
    PLIC + 0x201000 + (hart)*0x2000
}
fn PLIC_MCLAIM(hart : u64) -> u64 {
    PLIC + 0x200004 + (hart)*0x2000
}
fn PLIC_SCLAIM(hart : u64) -> u64 {
    PLIC + 0x201004 + (hart)*0x2000
}

// the kernel expects there to be RAM
// for use by the kernel and user pages
// from physical address 0x80000000 to PHYSTOP.
pub static KERNBASE : u64 =  0x80000000;

pub static PHYSTOP : u64 = KERNBASE + 128*1024*1024;

// map the trampoline page to the highest address,
// in both user and kernel space.
use super::riscv::{MAXVA, PGSIZE};
pub static TRAMPOLINE : u64 = MAXVA - PGSIZE;

// map kernel stacks beneath the trampoline,
// each surrounded by invalid guard pages.
fn KSTACK(p : u64) -> u64 {
    TRAMPOLINE - ((p)+1)* 2*PGSIZE
}

// User memory layout.
// Address zero first:
//   text
//   original data and bss
//   fixed-size stack
//   expandable heap
//   ...
//   TRAPFRAME (p->trapframe, used by the trampoline)
//   TRAMPOLINE (the same page as in the kernel)
pub static TRAPFRAME : u64 = TRAMPOLINE - PGSIZE;
