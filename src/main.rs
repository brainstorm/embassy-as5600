#![no_std]
#![no_main]

use panic_halt as _;

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    loop {}
}
