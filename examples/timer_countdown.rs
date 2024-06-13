#![no_main]
#![no_std]


use nb::block;

use aemics_stm32g4xx_hal::preludes::{
    default::*,
    digital::*,
    timers::*,
};

#[entry]
fn main() -> ! {
    //Load device peripherals.
    let dp = stm32::Peripherals::take().unwrap();

    //Grab abstracted RCC peripheral. This also initializes it to the default setting.
    let mut rcc = dp.RCC.constrain();

    //Initialize GPIOB objects. This splits the GPIOB register into individually accessible pins.
    let gpiob = dp.GPIOB.split(&mut rcc);

    //Grab pin B7 and convert it to a push-pull output pin. This is the pin connected to the LED on the PYglet board.
    let mut led = gpiob.pb7.into_push_pull_output();
    led.set_high().unwrap();

    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);

    let mut countdown = timer2.start_count_down_ms(5000.millis());

    block!(countdown.wait()).unwrap();

    led.set_low().unwrap();

    loop{

    }
}