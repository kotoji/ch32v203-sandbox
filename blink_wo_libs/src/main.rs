#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

core::arch::global_asm!(include_str!("startup.S"));

mod registers;
use crate::registers::*;

fn set_sys_clock_to_144_hsi() {
    unsafe {
        modify_u32(EXTEN_CTR, |v| v | EXTEN_PLL_HSI_PRE);

        // HCLK = SYSCLK
        modify_u32(RCC_CFGR0, |v| v | RCC_HPRE_DIV1);
        // PCLK2 = HCLK
        modify_u32(RCC_CFGR0, |v| v | RCC_PPRE2_DIV1);
        // PCLK1 = HCLK
        modify_u32(RCC_CFGR0, |v| v | RCC_PPRE1_DIV2);

        // PLL configuration: PLLCLK = HSI * 18 = 144 MHz
        modify_u32(RCC_CFGR0, |mut v| {
            v &= !(RCC_PLLSRC | RCC_PLLXTPRE | RCC_PLLMULL);
            v |= RCC_PLLSRC_HSI_DIV2 | RCC_PLLMULL18;
            v
        });

        // Enable PLL
        modify_u32(RCC_CTLR, |v| v | RCC_PLLON);
        // Wait till PLL is ready
        while read_u32(RCC_CTLR) & RCC_PLLRDY == 0 {}
        // Select PLL as system clock source
        modify_u32(RCC_CFGR0, |mut v| {
            v &= !RCC_SW;
            v |= RCC_SW_PLL;
            v
        });
        // Wait till PLL is used as system clock source
        while read_u32(RCC_CFGR0) & RCC_SWS != 0x08 {}
    }
}

fn system_init() {
    unsafe {
        modify_u32(RCC_CTLR, |v| v | RCC_HSI_ON);
        modify_u32(RCC_CFGR0, |v| v & 0xF8FF_0000);
        modify_u32(RCC_CTLR, |v| v & 0xFEF6_FFFF);
        modify_u32(RCC_CTLR, |v| v & 0xFFFB_FFFF);
        modify_u32(RCC_CFGR0, |v| v & 0xFF80_FFFF);

        write_u32(RCC_INTR, 0x009F_0000);
    }
    set_sys_clock_to_144_hsi();
}

fn led_init() {
    unsafe {
        // Enable GPIOA clock
        modify_u32(RCC_APB2PCENR, |v| v | RCC_APB2PERIPH_GPIOA);

        // Clear PA5 config (4 bits)
        modify_u64(GPIOA_CFGR, |v| v & !(0xF << (5 * 4)));
        // Set PA5 as push-pull output and max speed 50 MHz
        modify_u64(GPIOA_CFGR, |v| v | (0b0011 << (5 * 4)));
    }
}

fn led_set(enabled: bool) {
    unsafe {
        if enabled {
            modify_u32(GPIOA_OUTDR, |v| v | (1 << 5));
        } else {
            modify_u32(GPIOA_OUTDR, |v| v & !(1 << 5));
        }
    }
}

fn systick_delay(delay_ms: u32) {
    let counter = (delay_ms * (144_000_000 / 1000)) as u64;

    unsafe {
        // clear flag
        write_u32(SYSTICK_SR, 0);

        // initialize register
        write_u32(SYSTICK_CTLR, 0);

        // set counter
        write_u64(SYSTICK_CNT, counter);

        // use HCLK as clock source
        modify_u32(SYSTICK_CTLR, |v| v | SYSTICK_STCTL);
        // set downcount mode
        modify_u32(SYSTICK_CTLR, |v| v | SYSTICK_MODE);
        // enable systick
        modify_u32(SYSTICK_CTLR, |v| v | SYSTICK_STE);

        // wait for the counter to reach zero
        while (read_u32(SYSTICK_SR) & 0x01) != 0x01 {
            core::arch::asm!("nop");
        }

        // clear flag
        write_u32(SYSTICK_SR, 0);
        // disable systick
        modify_u32(SYSTICK_CTLR, |v| v & !SYSTICK_STE);
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    system_init();

    led_init();

    loop {
        led_set(true);
        systick_delay(1000);
        led_set(false);
        systick_delay(1000);
    }
}
