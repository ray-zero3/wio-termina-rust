#![no_std]
#![no_main]

use panic_halt as _;
use support::*;

#[entry]
fn main() -> ! {
    let (mut led, mut delay) = init();

    loop {
        led.toggle();
        delay.delay_ms(200u32);
    }
}
