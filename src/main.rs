#![no_std]
#![no_main]

use panic_halt as _;

use attiny_hal::clock::*;
use attiny_hal::delay::Delay;

//#[no_mangle]
fn main() -> ! {
  let dp = attiny_hal::Peripherals::take().unwrap();
  let pins = attiny_hal::pins!(dp);

  let mut led = pins.pb0.into_output();

  let mut delay = Delay::<MHz16>::new();

  loop {
    led.toggle();

    //delay.delay_ms(1000);
  }
}
