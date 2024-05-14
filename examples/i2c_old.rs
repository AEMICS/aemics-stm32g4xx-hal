#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use hal::i2c_compat::Config;
use hal::prelude::*;
use hal::stm32;
use hal::time::RateExtU32;
use aemics_stm32g4xx_hal as hal;

use cortex_m_rt::entry;
use log::info;


#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split(&mut rcc);

    let sda = gpioc.pc7.into_alternate_open_drain();
    let scl = gpioc.pc6.into_alternate_open_drain();

    let mut i2c = dp.I2C4.i2c(sda, scl, Config::new(40.kHz()), &mut rcc);
    // Alternatively, it is possible to specify the exact timing as follows (see the documentation
    // of with_timing() for an explanation of the constant):
    //let mut i2c = dp
    //  .I2C1
    //   .i2c(sda, scl, Config::with_timing(0x3042_0f13), &mut rcc);


    let buf: [u8; 1] = [0];
    loop {
        match i2c.write(0x50, &buf) {
            Ok(_) => info!("ok"),
            Err(err) => info!("error: {:?}", err),
        }
    }
}
