MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 32K
    RAM :   ORIGIN = 0x1FFFFC00, LENGTH = 4K
}

ENTRY(reset_handler);

EXTERN(RESET_VECTOR);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        // Stack pointer
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        // Reset vector
        KEEP(*(.vector_table.reset_vector));
    } > FLASH

    .text :
    {
        *(.text .text.*);
    } > FLASH

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
    }
}
