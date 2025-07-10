#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal::hal::ehal::digital::InputPin;
use wio_terminal::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;
use wio_terminal as wio;
use wio::entry;
use wio::pac::Peripherals;
use wio::Pins;
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let sets = Pins::new(peripherals.port).split();
    let mut led = sets.user_led.into_push_pull_output();
    let mut button1 = sets.buttons.button1.into_pull_up_input();

    loop {
        if button1.is_low().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
