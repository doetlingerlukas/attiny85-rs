#![no_std]
#![no_main]

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use panic_halt as _;

use attiny_hal::clock::*;
use attiny_hal::delay::Delay;

#[no_mangle]
fn main() -> ! {
  let dp = attiny_hal::Peripherals::take().unwrap();
  let pins = attiny_hal::pins!(dp);

  let mut led = pins.pb0.into_output();

  let mut delay = Delay::<MHz16>::new();

  loop {
    led.toggle();

    delay.delay_us(1000u16);
  }
}
