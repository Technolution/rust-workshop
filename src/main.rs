#![no_std]
#![no_main]

extern crate panic_halt;

use rp_pico::entry;
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_r = pins.gpio12.into_push_pull_output();
    let mut led_g = pins.gpio13.into_push_pull_output();
    let mut led_b = pins.gpio14.into_push_pull_output();
    let mut led_y = pins.gpio15.into_push_pull_output();

    led_r.set_high().unwrap();
    led_g.set_high().unwrap();
    led_b.set_high().unwrap();
    led_y.set_high().unwrap();

    loop {}
}
