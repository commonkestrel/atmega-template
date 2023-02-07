#![no_std]

use atmega::prelude::*;

pub fn init_button(pin: Pin) {
    pin_mode(pin, PinMode::INPUT_PULLUP);
}

pub fn button_pressed(pin: Pin) -> bool {
    !digital_read(pin)
}