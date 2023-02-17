#         # qemu -kernel loads the kernel at 0x80000000
#         # and causes each hart (i.e. CPU) to jump there.
#         # kernel.ld causes the following code to
#         # be placed at 0x80000000.
# .section .text.entry
# .global _entry
# _entry:
#         # set up a stack for C.
#         # stack0 is declared in start.c,
#         # with a 4096-byte stack per CPU.
#         # sp = stack0 + (hartid * 4096)
#         addi sp, sp, 100 
#         li a0, 1024 * 4
#         csrr a1, mhartid
#         addi a1, a1, 1
#         #mul a0, a0, a1
#         add sp, sp, a0
#         # jump to start() in start.c
#         call start

# spin:
#         j spin

    .section .text.entry
    .globl _entry
_entry:
    la sp, boot_stack_top
    call start

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: