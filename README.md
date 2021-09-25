# attiny85-rs

Running Rust on a [Digispark USB Development Board](http://digistump.com/products/1), which uses an Attiny85 microcontroller.

## Requirements

Obviously Rust is required to compile the project. Due to a problem with newer Rust versions in `avr-hal`, the environment is configured to use the "nightly-2021-01-07" channel.

The following is also needed to get the program on the microcontroller:

- `avr-gcc` and `avr-objcopy`
- [micronucleus](https://github.com/micronucleus/micronucleus) binary (place in `micronucleus` folder for the `Rakefile` to work)

Make sure to also install drivers for bootloader, which you can find [here](https://github.com/digistump/DigistumpArduino/releases).

## How to run

```sh
rake flash
```
