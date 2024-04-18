#![no_main]
#![no_std]

use panic_semihosting as _;
use aemics_stm32g4xx_hal as aemics_hal;

//Hier importeren we de benodigde functies uit de HAL die we nodig hebben
use aemics_hal::{
    gpio::GpioExt,
    prelude::*,
    rcc::{Config, RccExt},
    stm32::{self, interrupt, Interrupt},
    timer::{CountDownTimer, Event, Timer},
    time::*,
};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::{asm::wfi, interrupt::Mutex};
use cortex_m_rt::entry;
use aemics_stm32g4xx_hal::pwr::PwrExt;

//We gebruiken hier een Mutex voor de Timer. dit zorgt er voor dat maar 1 proces tegelijkertijd de waarde kan veranderen.
static TIMER_TIM2: Mutex<RefCell<Option<CountDownTimer<stm32::TIM2>>>> =
    Mutex::new(RefCell::new(None));

//Hier komt de code voor de interrupt. Eerst zetten we de interrupt bit weer uit, daarna herstart de timer.
#[interrupt]
fn TIM2() {
    //Laadt de timer in de interrupt
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut t2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            //Hier resetten we de interrupt
            t2.clear_interrupt(Event::TimeOut);
        }
    });
}

#[entry]
fn main() -> ! {
    //Peripherals in laden
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi(), dp.PWR.constrain().freeze());

    //Pin goed zetten
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();

    //Timer initialiseren
    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);

    //Timer starten
    let mut timer2_i = timer2.start_count_down_ms(5000.millis());
    timer2_i.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| TIMER_TIM2.borrow(cs).borrow_mut().replace(timer2_i));

    //interrupt mogelijk maken
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    loop {
        //We hoeven alleen te wachten op een interrupt, en dan de led te veranderen
        wfi();
        led.toggle().unwrap();
    }
}
