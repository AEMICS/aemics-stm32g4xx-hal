// This example is to test the SPI without any external devices.
// It puts "Hello world!" on the mosi-line and logs whatever is received on the miso-line to the info level.
// The idea is that you should connect miso and mosi, so you will also receive "Hello world!".

#![no_main]
#![no_std]

use fugit::{RateExtU32};
use aemics_stm32g4xx_hal as aemics_hal;


use aemics_hal::preludes::{
    default::*,
    digital::*,
    delay::*,
    timers::*,
    spi::*
};

use aemics_stm32g4xx_hal::gpio::AF5;


#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let pwr = dp.PWR.constrain().freeze();
    let mut rcc = rcc.freeze(Config::hsi(), pwr);
    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);
    let mut delay_tim2 = DelayFromCountDownTimer::new(timer2.create_count_down_ms());

    let gpiob = dp.GPIOB.split(&mut rcc);

    //Create and configure the SCLK, MISO and MOSI pins. In this case, AF5 is the alternate function of these pins that allows them to act as SPI pins.
    let sclk = gpiob.pb13.into_alternate::<AF5>();
    let miso = gpiob.pb14.into_alternate::<AF5>();
    let mosi = gpiob.pb15.into_alternate::<AF5>();
    //Alternatively, it is possible to be implicit with the into_alternate method, and have the Rust compiler infer the required alternate function:
    //  let sclk = gpiob.pb13.into_alternate();
    //  let miso = gpiob.pb14.into_alternate();
    //  let mosi = gpiob.pb15.into_alternate();
    //

    let mut spi = dp
        .SPI2
        .spi((sclk, miso, mosi), MODE_0, 400.kHz(), &mut rcc);

    let mut cs = gpiob.pb12.into_push_pull_output();
    cs.set_high().unwrap();

    let message: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut received_bytes: [u8; 12] = [0; 12];

    loop {
        cs.set_low().unwrap();

        spi.transfer(&mut received_bytes, &message).unwrap();

        cs.set_high().unwrap();

        delay_tim2.delay_ms(1000);
    }
}
