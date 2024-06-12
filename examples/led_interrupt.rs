#![no_main]
#![no_std]


use aemics_stm32g4xx_hal::preludes::{
    default::*,
    digital::*,
    interrupts::*,
    timers::*,
};

use aemics_stm32g4xx_hal::pwr::PwrExt;
use panic_semihosting as _; //Panic Handler


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
