#![no_std]
#![no_main]

use as5600::As5600;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy::{self},
    gpio::Io,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::TimerGroup,
};

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio14,
        50.kHz(),
        &clocks,
    );

    let mut encoder = As5600::new(i2c);

    loop {
        let status = encoder.angle();
        esp_println::println!("{:?}", status);
    }
}
