#![no_main]
#![no_std]


use aemics_stm32g4xx_hal::preludes::{
    default::*,
    digital::*,
    delay::*,
};

use panic_semihosting as _; //Panic Handler

#[entry]
fn main() -> ! {
    //Load device and core peripherals.
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //Grab abstracted RCC peripheral. This also initializes it to the default setting.
    let mut rcc = dp.RCC.constrain();

    //Initialize GPIOB objects. This splits the GPIOB register into individually accessible pins.
    let gpiob = dp.GPIOB.split(&mut rcc);

    //Grab pin B7 and convert it to a push-pull output pin. This is the pin connected to the LED on the PYglet board.
    let mut led = gpiob.pb7.into_push_pull_output();


    let mut delay = cp.SYST.delay(&rcc.clocks);

    //Program, toggles the LED on/off at 1Hz.
    loop {

        led.set_high().unwrap();

        delay.delay_ms(100);

        led.set_low().unwrap();

        delay.delay_ms(100);
    }
}