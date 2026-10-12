#![no_std]
#![allow(unused)]

#[inline(always)]
pub unsafe fn read_u32(addr: *const u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr) }
}

#[inline(always)]
pub unsafe fn write_u32(addr: *mut u32, value: u32) {
    unsafe { core::ptr::write_volatile(addr, value) }
}

#[inline(always)]
pub unsafe fn read_u64(addr: *const u64) -> u64 {
    unsafe { core::ptr::read_volatile(addr) }
}

#[inline(always)]
pub unsafe fn write_u64(addr: *mut u64, value: u64) {
    unsafe { core::ptr::write_volatile(addr, value) }
}

#[inline(always)]
pub unsafe fn modify_u32<F>(addr: *mut u32, f: F)
where
    F: FnOnce(u32) -> u32,
{
    unsafe {
        let v = read_u32(addr as *const u32);
        write_u32(addr, f(v));
    }
}

#[inline(always)]
pub unsafe fn modify_u64<F>(addr: *mut u64, f: F)
where
    F: FnOnce(u64) -> u64,
{
    unsafe {
        let v = read_u64(addr as *const u64);
        write_u64(addr, f(v));
    }
}

// registers

pub const RCC_CTLR: *mut u32 = 0x4002_1000 as *mut u32;
pub const RCC_CFGR0: *mut u32 = 0x4002_1004 as *mut u32;
pub const RCC_INTR: *mut u32 = 0x4002_1008 as *mut u32;

pub const EXTEN_CTR: *mut u32 = 0x4002_3800 as *mut u32;

// HSI clock input mode for PLL
pub const EXTEN_PLL_HSI_PRE: u32 = 1 << 4;

pub const RCC_CTLR_PLLON: u32 = 1 << 24;
pub const RCC_HSI_ON: u32 = 0x0000_0001;
pub const RCC_HPRE_DIV1: u32 = 0x0000_0000;
pub const RCC_PPRE2_DIV1: u32 = 0x0000_0000;
pub const RCC_PPRE1_DIV2: u32 = 0x0000_0400;
pub const RCC_PLLSRC: u32 = 1 << 16;
pub const RCC_PLLXTPRE: u32 = 1 << 17;
pub const RCC_PLLMULL: u32 = 0x003C_0000;
pub const RCC_PLLSRC_HSI_DIV2: u32 = 0x0000_0000;
pub const RCC_PLLMULL18: u32 = 0x003C_0000;
pub const RCC_PLLON: u32 = 0x0100_0000;
pub const RCC_PLLRDY: u32 = 0x0200_0000;
pub const RCC_SW: u32 = 0x0000_0003;
pub const RCC_SW_PLL: u32 = 0x0000_0002;
pub const RCC_SWS: u32 = 0x0000000C;

pub const RCC_APB2PCENR: *mut u32 = 0x4002_1018 as *mut u32;
pub const RCC_APB2PERIPH_GPIOA: u32 = 1 << 2;

pub const GPIOA_CFGR: *mut u64 = 0x4001_0800 as *mut u64;
pub const GPIOA_OUTDR: *mut u32 = 0x4001_080C as *mut u32;

pub const SYSTICK_CTLR: *mut u32 = 0xE000_F000 as *mut u32;
pub const SYSTICK_SR: *mut u32 = 0xE000_F004 as *mut u32;
pub const SYSTICK_CNTL: *mut u32 = 0xE000_F008 as *mut u32;
pub const SYSTICK_CNTH: *mut u32 = 0xE000_F00C as *mut u32;
pub const SYSTICK_CNT: *mut u64 = SYSTICK_CNTL as *mut u64;
pub const SYSTICK_CMPLR: *mut u32 = 0xE000_F010 as *mut u32;
pub const SYSTICK_CMPHR: *mut u32 = 0xE000_F014 as *mut u32;
pub const SYSTICK_CMPR: *mut u64 = SYSTICK_CMPLR as *mut u64;

// Initialize counter value
pub const SYSTICK_INIT: u32 = 1 << 5;
// Mode (0: Upcount, 1: Downcount)
pub const SYSTICK_MODE: u32 = 1 << 4;
// Counter auto reload
pub const SYSTICK_STRE: u32 = 1 << 3;
// Clock source selector
pub const SYSTICK_STCTL: u32 = 1 << 2;
// SysTick Enabled
pub const SYSTICK_STE: u32 = 1 << 0;
