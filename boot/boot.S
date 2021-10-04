MULTIBOOT_ALIGN EQU 1<<0                                      ; Align loaded modules on page boundaries
MULTIBOOT_MEMINFO EQU 1<<1                                    ; Provide memory map
MULTIBOOT_FLAGS EQU MULTIBOOT_ALIGN | MULTIBOOT_MEMINFO       ; Multiboot flags
MULTIBOOT_MAGIC EQU 0x1BADB002                                ; Multiboot 1 magic number
MULTIBOOT_CHECKSUM EQU -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS) ; Checksum

SECTION .multiboot
ALIGN 4
DD MULTIBOOT_MAGIC
DD MULTIBOOT_FLAGS
DD MULTIBOOT_CHECKSUM

SECTION .bss
ALIGN 16
stack_bottom:
RESB 16384 ; 16 KiB stack
stack_top:


SECTION .text
GLOBAL _start:function (_start.end - _start)
_start:
    MOV esp, stack_top
    EXTERN kmain
    CALL kmain
    CLI
.hang: HLT
    JMP .hang
.end:







