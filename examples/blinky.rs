#![no_main]
#![no_std]


//Import the HAL.
use aemics_stm32g4xx_hal as aemics_hal;
use aemics_hal::hal_api::delay::DelayNs;

//Import version specific digital logic. (This API changed between embedded-hal v0.2.7 and v1.0.0)
use aemics_hal::hal_api::digital::*;

use aemics_hal::{
    delay::*,
    gpio::GpioExt,
    rcc::RccExt,
};

//Import peripheral library of the STM32G4 family.
use aemics_hal::stm32;

//Import the core peripherals of the cortex-m architecture. This allows access to the system timer (SYST) for example.
use aemics_hal::cortex_m;

//Required for targeting a cortex-m platform with Rust code. Handles memory layout, startup, etc.
use aemics_hal::cortex_m_rt::entry;


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

    //Create a delay provider. This is driven by the system timer (SysTick)
    //Create a delay provider. This is driven by the system timer (SysTick)
    let timer2 = dp
        .TIM2
        .timer(100.ms(), dp.TIM2, &mut rcc.clocks);
    let mut delay = DelayFromCountDownTimer::new(timer2);

    //Program, toggles the LED on/off at 1Hz.
    loop {

        led.set_high().unwrap();

        delay.delay_ms(100);

        led.set_low().unwrap();

        delay.delay_ms(100);
    }
}