ENTRY( _start )

__stack_size = 2048;

PROVIDE( _stack_size = __stack_size );

MEMORY
{
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 224K
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  PROVIDE( _bootloader_limit = ORIGIN(FLASH) + LENGTH(FLASH) );

  .init :
  {
    _sinit = .;
    . = ALIGN(4);
    KEEP(*(SORT_NONE(.init)))
    . = ALIGN(4);
    _einit = .;
  } >FLASH AT>FLASH

  .vector :
  {
    *(.vector);
    . = ALIGN(64);
  } >FLASH AT>FLASH

  .text :
  {
    . = ALIGN(4);
    *(.text .text.*)
    *(.rodata .rodata.*)
    . = ALIGN(4);
  } >FLASH AT>FLASH

  .dalign :
  {
    . = ALIGN(4);
    PROVIDE( _data_vma = . );
  } >RAM AT>FLASH

  .dlalign :
  {
    . = ALIGN(4);
    PROVIDE( _data_lma = . );
  } >FLASH AT>FLASH

  .data :
  {
    *(.data .data.*)
    . = ALIGN(8);
    PROVIDE( __global_pointer$ = . + 0x800);
    *(.sdata .sdata.*)
    . = ALIGN(8);
    *(.srodata .srodata.*)
    . = ALIGN(4);
    PROVIDE( _edata = . );
  } >RAM AT>FLASH

  .bss (NOLOAD) :
  {
    . = ALIGN(4);
    PROVIDE( _sbss = . );
    *(.bss*)
    . = ALIGN(4);
    PROVIDE( _ebss = . );
  } >RAM

  .stack ORIGIN(RAM) + LENGTH(RAM) - __stack_size (NOLOAD):
  {
    PROVIDE( _heap_end = . );
    . = ALIGN(4);
    PROVIDE( _susrstack = . );
    . = . + __stack_size;
    PROVIDE( _eusrstack = . );
  } >RAM
}
