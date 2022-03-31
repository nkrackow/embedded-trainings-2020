#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[entry]
fn main() -> ! {
    // board initialization
    let board = dk::init().unwrap();

    let mut led = board.leds._1;

    defmt::println!("Hello, world!");

    loop {
        // asm::bkpt();
        asm::delay(1000000);
        led.toggle();
    }
}
