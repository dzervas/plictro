/* Got from https://github.com/adafruit/Adafruit_nRF52_Arduino/blob/master/cores/nRF5/linker/nrf52840_s140_v6.ld */
MEMORY
{
  FLASH (rx)  : ORIGIN = 0x00026000, LENGTH = 0xED000 - 0x26000
  RAM (rwx) : ORIGIN = 0x20006000, LENGTH = 0x20040000 - 0x20006000
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
