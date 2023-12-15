#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

mod delay;
mod heap;
mod led;
mod push_button;

use delay::init_delay;
use heap::init_heap;
use led::Led;
use push_button::PushButton;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);

    // Initialize the heap
    init_heap();

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up Delay
    // Delay takes ownership of the clock and timer hardware and provides an abstraction that can be used to sleep for certain amounts of time
    // For example, call `delay.delay_ms(10)` to sleep for 10ms.
    let mut delay = init_delay(
        pac.WATCHDOG,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        core.SYST,
        &mut pac.RESETS,
    );

    // Set up the led pins to correspond to their GPIO pin, into output mode
    // Note that the gpio pin numbers don't correspond to the physical pin numbers
    // See: https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#pinout-and-design-files
    let mut led_r = Led::new(pins.gpio12); // pin 16
    let mut led_g = Led::new(pins.gpio13); // pin 17
    let mut led_b = Led::new(pins.gpio14); // pin 19
    let mut led_y = Led::new(pins.gpio15); // pin 20

    // Create buttons
    let but_r = PushButton::new(pins.gpio19); // pin 25
    let but_g = PushButton::new(pins.gpio18); // pin 24
    let but_b = PushButton::new(pins.gpio17); // pin 22
    let but_y = PushButton::new(pins.gpio16); // pin 21

    // We enter a loop
    loop {
        led_r.set_led(but_r.is_pressed());
        led_g.set_led(but_g.is_pressed());
        led_b.set_led(but_b.is_pressed());
        led_y.set_led(but_y.is_pressed());
        delay.delay_ms(1);
    }
}
