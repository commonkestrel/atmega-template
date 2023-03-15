#![no_std]
#![no_main]

use atmega::prelude::*;

run!(setup, run);

/// Equivalent to the `setup` function in the Arduino language
fn setup() {
    // Set pin 9 to output
    pin_mode(Pin::D9, PinMode::OUTPUT);
}

/// Equivalent to the `loop` function in the Arduino language
fn run() {
    // Toggle pin 9
    digital_toggle(Pin::D9);
    // Wait 1 second
    delay(1000)
}
