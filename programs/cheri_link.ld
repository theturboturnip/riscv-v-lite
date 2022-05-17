/*
 * The stack starts at the top of memory, defined by __sp, and grows down.
 * Unfortunately this means we must hardcode the size of memory.
 * Here we choose 64K. Change this value to change the working memory size.
 */
__sp = 0x10000 - 32;
outputAttempted = 0x01000000;
outputSucceeded = 0x01000008;
finished = 0x01000010;
ramBoundary = 0x25000;

MEMORY
{
    RAM (rwx) : ORIGIN = 0, LENGTH = 256k
    IO (w) : org = 0x01000000, len = 16
}

SECTIONS
{
    __cap_relocs : {
        *(__cap_relocs)
    } > RAM

    .text.init : {
        *(.text.init)
    } > RAM

    .text : {
        *(.text)
    } > RAM

    .data : {
        *(.sbss)
        *(COMMON)
        *(.bss)
        *(.sdata)
        *(.rodata*)
        *(.data*)
        *(.got*)
    } > RAM
}