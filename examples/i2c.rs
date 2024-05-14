#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

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

    let gpiob = dp.GPIOB.split(&mut rcc);

    //Grab pin B7 and convert it to a push-pull output pin. This is the pin connected to the LED on the PYglet board.
    let mut led = gpiob.pb7.into_push_pull_output();


    let mut delay = cp.SYST.delay(&rcc.clocks);

    let buf: [u8; 2] = [0, 1];
    loop {
        match i2c.write(0b1010001_u8, &buf) {
            Ok(_) => {
                led.toggle().unwrap();
                delay.delay_ms(100);
            }
            Err(err) => info!("error: {:?}", err),
        }
    }
}
