#![no_std]
#![no_main]

use as5600::As5600;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, embassy, gpio::Io, i2c::I2C, peripherals::Peripherals, prelude::*, system::SystemControl, timer::TimerGroup
};


#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);
    esp_println::println!("Bing!");
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    
    let i2c = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio41,
        io.pins.gpio42,
        400.kHz(),
        &clocks,
    );
    let delay = Delay::new(&clocks);
    
    let mut encoder = As5600::new(i2c);
    loop {
        if let Ok(angle) = encoder.raw_angle() {
            esp_println::println!("{:?}", angle);
            delay.delay_millis(300);
        };
        if let Ok(magnet) = encoder.magnet_status() {
            esp_println::println!("{:?}", magnet);
            delay.delay_millis(300);
        };
    }
}
