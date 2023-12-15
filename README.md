# Hardware
For pin layout of the RPI pico refer to: https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#pinout-and-design-files

One side of the breadboard will be used for the leds, the other side for the buttons.

## Hardware required (Provided):
- 1 Raspberry Pi Pico
- 1 USB Micro Cable
- 1 Breadboard
- 4x 220 Ohm resistor
- 1x Red led
- 1x Green led
- 1x Blue led
- 1x Yellow led
- 4x Push button
- 10 M-F jumper
- 4x M-M jumper

![](readme-images/rpi.jpg)

## Leds
- Connect pin 16,17,19,20 to the red,green,blue,yellow to the positive side of the LEDs.
- Connect negative side of the leds to the center column using the resistors.
- Connect center column to ground at pin 18. 
- Your hardware should now look like this:

![](readme-images/leds.jpg)



## 


# Software

Installation:
- Install VS code with the rust plugin
- Install rustup
- `rustup target add thumbv6m-none-eabi`
- `cargo install elf2uf2-rs`

Running:
- Unplug rpi
- Hold bootsel while plugging in
- `cargo run`



