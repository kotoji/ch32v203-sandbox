#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use hal::gpio::{Level, Output};
use {ch32_hal as hal, panic_halt as _};

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    let p = hal::init(hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSI,
        ..Default::default()
    });

    let mut led = Output::new(p.PA5, Level::Low, Default::default());
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}
