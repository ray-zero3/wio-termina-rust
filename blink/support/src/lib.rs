#![no_std]

use wio_terminal as wio;

pub use wio::entry;
pub use embedded_hal::delay::DelayNs;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::gpio::{Pin, PA15, Output, PushPull, Disabled, Floating};
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

pub struct Led {
    pin:  Pin<PA15, Output<PushPull>>
}

impl Led {
    fn new(pin: Pin<PA15, Disabled<Floating>>) -> Led {
        Led { pin: pin.into_push_pull_output(), }
    }

    /// LEDを点灯する
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// LEDを消灯する
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    /// LEDが点灯していれば消灯し、消灯していれば点灯する
    pub fn toggle(&mut self) {
        self.pin.toggle().ok();
    }
}

pub fn init() -> (Led, Delay) {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let delay = Delay::new(core.SYST, &mut clocks);

    let sets = wio::Pins::new(peripherals.port).split();
    let led = Led::new(sets.user_led);

    (led, delay)
}
