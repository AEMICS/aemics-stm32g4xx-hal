#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

//! I2C example
//!
//! This example shows how to use the read and write functionality of the I2C API.
//! The example assumes the usage of an EEPROM chip (24AA04 4K I2C Serial EEPROM)
//!
//! This example uses 7-bit addressing (the default for I2C). However, the [`I2C API`](aemics_hal::hal_api::i2c) also supports 10-bit addressing.
//!
//! Using the read/write functionality remains the same across any device.
//!
//! See [`examples/i2c_transaction.rs`](C:\Users\wybre\Documents\Saxion\YearFour\Graduation\projects\stm32g4xx_hal\examples\i2c_transaction.rs) for a more complex setup using the [`transaction`](I2c::transaction) functionality.
//!


use aemics_stm32g4xx_hal as aemics_hal;

use aemics_hal::preludes::default::*;
use aemics_hal::preludes::digital::*;
use aemics_hal::preludes::i2c::*;
use aemics_hal::preludes::delay::*;
use log::info;

#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split(&mut rcc);

    let sda = gpioc.pc7.into_alternate_open_drain();
    let scl = gpioc.pc6.into_alternate_open_drain();

    let mut i2c = dp.I2C4.i2c(sda, scl, aemics_hal::i2c::Config::new(40.kHz()), &mut rcc);
    // Alternatively, it is possible to specify the exact timing as follows (see the documentation
    // of with_timing() for an explanation of the constant):
    //let mut i2c = dp
    //  .I2C1
    //   .i2c(sda, scl, Config::with_timing(0x3042_0f13), &mut rcc);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    let write_buf: [u8; 4] = [0, 0, 0, 0];
    let mut read_buf: [u8; 10] = [0; 10];
    loop {
        match i2c.write(0b1010000_u8, &write_buf) {
            Ok(_) => info!("Ok"),
            Err(err) => {
                info!("error: {:?}", err);
            },
        }

        delay.delay_ms(5); //Delay 5 milliseconds to allow the EEPROM chip to do its page write internal write process.

        match i2c.read(0b1010000_u8, &mut read_buf) {
            Ok(_) => info!("Ok"),
            Err(err) => {
                info!("error: {:?}", err);
            },
        }
    }
}
