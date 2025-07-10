#![no_std]
#![no_main]
#![allow(dead_code)]


use panic_halt as _;
use wio_terminal::hal::ehal::digital::InputPin;
use wio_terminal::hal::gpio::{Input, Output, Pin, PullUp, PushPull, PA15, PC26};
use wio_terminal::prelude::{_atsamd_hal_embedded_hal_digital_v2_OutputPin, _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin};
use wio_terminal as wio;
use wio::entry;
use wio::pac::Peripherals;
use wio::Pins;


struct Button1 {
    pin: Pin<PC26, Input<PullUp>>,
}

impl Button1 {
    pub fn new(pin: Pin<PC26, Input<PullUp>>) -> Self {
        Button1 { pin }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap()
    }

    pub fn is_released(&mut self) -> bool {
        self.pin.is_high().unwrap()
    }
}

struct Led {
    pin: Pin<PA15, Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pin<PA15, Output<PushPull>>) -> Self {
        Led { pin }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }
    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
    fn toggle(&mut self) {
        self.pin.toggle().unwrap();
    }
}


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let sets = Pins::new(peripherals.port).split();
    let mut
    button1 = Button1::new(sets.buttons.button1.into_pull_up_input());
    let mut led = Led::new(sets.user_led.into_push_pull_output());

    loop {
        if button1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}
