#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

//use panic_halt as _;

use arduino_hal::{ pins, prelude::_unwrap_infallible_UnwrapInfallible, I2c, Peripherals };
use as5600::As5600;

use embassy_executor::Spawner;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .unwrap_infallible();
    }

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

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
        let status = encoder.angle().unwrap();
        ufmt::uwriteln!(&mut serial, "{}", "Angle has been read").unwrap_infallible();
        ufmt::uwriteln!(&mut serial, "{}", status).unwrap_infallible();
    }
}
