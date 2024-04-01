#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use panic_halt as _;

use arduino_hal::{ pins, prelude::_unwrap_infallible_UnwrapInfallible, I2c, Peripherals };
use as5600::As5600;

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);                                                                        
    let i2c = I2c::new(
        peripherals.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let mut encoder = As5600::new(i2c);

    loop {
        ufmt::uwriteln!(&mut serial, "{}", "Reading angle").unwrap();
        // let status = encoder.angle().unwrap();
        ufmt::uwriteln!(&mut serial, "{}", "Angle has been read").unwrap_infallible();
        // ufmt::uwriteln!(&mut serial, "{}", status).unwrap_infallible();
    }
}