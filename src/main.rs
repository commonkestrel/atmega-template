#![no_std]
#![no_main]

use atmega::prelude::*;

run!(setup, run)

struct State {

}

/// Equivalent to the `setup` function in the Arduino language
fn setup() -> State {
    // Set pin 9 to output
    pin_mode(Pin::D9, PinMode::OUTPUT);
    
    State {}
}

/// Equivalent to the `loop` function in the Arduino language
fn run(_state: &mut State) {
    // Toggle pin 9
    digital_toggle(Pin::D9);
    // Wait 1 second
    delay(1000)
}