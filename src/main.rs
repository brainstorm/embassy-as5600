#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use panic_halt as _;

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    loop {}
}
