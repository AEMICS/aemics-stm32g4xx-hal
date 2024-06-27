//! Delay providers
//!
//! There are currently two delay providers. In general you should prefer to use
//! [Delay](Delay), however if you do not have access to `SYST` you can use
//! [DelayFromCountDownTimer](DelayFromCountDownTimer) with any timer that
//! implements the [CountDown](embedded_hal::timer::CountDown) trait. This can be
//! useful if you're using [RTIC](https://rtic.rs)'s schedule API, which occupies
//! the `SYST` peripheral.
//!
//! # Examples
//!
//! ## Delay
//!
//! ```no_run
//! let rcc =  Peripherals::take().unwrap().contrain();
//! let rcc = rcc.freeze(Config::hsi());
//! let mut delay = cp.SYST.delay(&rcc.clocks);
//!
//! delay.delay(500.ms() );
//!
//! // Release SYST from the delay
//! let syst = delay.free();
//! ```
//!
//! ## DelayFromCountDownTimer
//!
//! ```no_run
//! let timer2 = device
//!     .TIM2
//!     .timer(100.ms(), device.peripheral.TIM2, &mut device.clocks);
//! let mut delay = DelayFromCountDownTimer::new(timer2);
//!
//! delay.delay_ms(500);
//!
//! // Release the timer from the delay
//! let timer2 = delay.free();
//! ```

use crate::rcc::Clocks;
use crate::time::{MicroSecond, MilliSecond, NanoSecond};
pub use cortex_m::delay::*;
use cortex_m::peripheral::SYST;

use crate::nb::block;
use crate::time::ExtU32;
use hal_api::delay::DelayNs;
use hal_api_old::blocking::delay::{DelayUs, DelayMs};

pub trait CountDown_ns: crate::hal_api_custom::timer::CountDownNs {
    fn max_period(&self) -> NanoSecond;
}

pub trait CountDownCompat: hal_api_old::timer::CountDown {
    fn max_period(&self) -> MicroSecond;
}

pub trait CountDown_us: crate::hal_api_custom::timer::CountDownUs {
    fn max_period(&self) -> MicroSecond;
}

pub trait CountDown_ms: crate::hal_api_custom::timer::CountDownMs {
    fn max_period(&self) -> MilliSecond;
}

pub trait SYSTDelayExt {
    fn delay(self, clocks: &Clocks) -> Delay;
}

impl SYSTDelayExt for SYST {
    fn delay(self, clocks: &Clocks) -> Delay {
        Delay::new(self, clocks.ahb_clk.raw())
    }
}

//Until the cortex-m crate starts supporting HAL 1.0, this is the best we can do. NanoSecond accurate delays are unsupported by this crate.
//TODO: If time allows, make custom fork of cortex-m crate and update to current HAL version.

pub trait DelayExt {
    
    fn delay_us<T>(&mut self, delay: T)
        where
            T: Into<MicroSecond>;

    fn delay_ms<T>(&mut self, delay: T)
        where
            T: Into<MilliSecond>;
}

///Extension for Delay functionality from cortex-m crate.
impl DelayExt for Delay {

    fn delay_us<T>(&mut self, delay: T)
        where
            T: Into<MicroSecond>,
    {
        self.delay_us(delay.into().to_micros())
    }

    fn delay_ms<T>(&mut self, delay: T)
        where
            T: Into<MilliSecond>,
    {
        self.delay_ms(delay.into().to_millis())
    }
}

/// CountDown Timer as a delay provider
pub struct DelayFromCountDownTimer<T>(T);

impl<T> DelayFromCountDownTimer<T> {
    /// Creates delay provider from a CountDown timer
    pub fn new(timer: T) -> Self {
        Self(timer)
    }

    /// Releases the Timer
    pub fn free(self) -> T {
        self.0
    }
}

macro_rules! impl_delay_from_count_down_timer  {
     ($Delay:ident, NanoSecond, $delay:ident ) => {
        impl<T> $Delay for DelayFromCountDownTimer<T>
        where
            T: CountDown_ns<Time = NanoSecond>,
        {
            fn $delay(&mut self, t: u32) {
                let mut time_left = t;

                let max_sleep = self.0.max_period();
                let max_sleep = max_sleep.to_nanos();

                if time_left > max_sleep {
                    self.0.start(max_sleep.nanos());

                    // Process the time one max_sleep duration at a time
                    // to avoid overflowing both u32 and the timer
                    for _ in 0..(time_left / max_sleep) {
                        block!(self.0.wait()).ok();
                        time_left -= max_sleep;
                    }
                }

                assert!(time_left <= u32::MAX);
                assert!(time_left <= max_sleep);

                let time_left: NanoSecond = (time_left).nanos();

                // Only sleep
                if time_left.ticks() > 0 {
                    self.0.start(time_left);
                    block!(self.0.wait()).ok();
                }
            }
        }
    };

    ($Delay:ident, MicroSecond, $delay:ident ) => {
        impl<T> $Delay for DelayFromCountDownTimer<T>
        where
            T: CountDown_us<Time = MicroSecond>,
        {
            fn $delay(&mut self, t: u32) {
                let mut time_left = t;

                let max_sleep = self.0.max_period();
                let max_sleep = max_sleep.to_micros();

                if time_left > max_sleep {
                    self.0.start(max_sleep.micros());

                    // Process the time one max_sleep duration at a time
                    // to avoid overflowing both u32 and the timer
                    for _ in 0..(time_left / max_sleep) {
                        block!(self.0.wait()).ok();
                        time_left -= max_sleep;
                    }
                }

                assert!(time_left <= u32::MAX);
                assert!(time_left <= max_sleep);

                let time_left: MicroSecond = (time_left).micros();

                // Only sleep
                if time_left.ticks() > 0 {
                    self.0.start(time_left);
                    block!(self.0.wait()).ok();
                }
            }
        }
    };

    ($Delay:ident, MilliSecond, $delay:ident ) => {
        impl<T> $Delay for DelayFromCountDownTimer<T>
        where
            T: CountDown_ms<Time = MilliSecond>,
        {
            fn $delay(&mut self, t: u32) {
                let mut time_left = t;

                let max_sleep = self.0.max_period();
                let max_sleep = max_sleep.to_millis();

                if time_left > max_sleep {
                    self.0.start(max_sleep.millis());

                    // Process the time one max_sleep duration at a time
                    // to avoid overflowing both u32 and the timer
                    for _ in 0..(time_left / max_sleep) {
                        block!(self.0.wait()).ok();
                        time_left -= max_sleep;
                    }
                }

                assert!(time_left <= u32::MAX);
                assert!(time_left <= max_sleep);

                let time_left: MilliSecond = (time_left).millis();

                // Only sleep
                if time_left.ticks() > 0 {
                    self.0.start(time_left);
                    block!(self.0.wait()).ok();
                }
            }
        }
    };
}

macro_rules! impl_delay_from_count_down_timer_old  {
    ($Delay:ident, MicroSecond, $delay:ident ) => {
        impl<T> $Delay<u32> for DelayFromCountDownTimer<T>
        where
            T: CountDown_us<Time = MicroSecond>,
        {
            fn $delay(&mut self, t: u32) {
                let mut time_left = t;

                let max_sleep = self.0.max_period();
                let max_sleep = max_sleep.to_micros();

                if time_left > max_sleep {
                    self.0.start(max_sleep.micros());

                    // Process the time one max_sleep duration at a time
                    // to avoid overflowing both u32 and the timer
                    for _ in 0..(time_left / max_sleep) {
                        block!(self.0.wait()).ok();
                        time_left -= max_sleep;
                    }
                }

                assert!(time_left <= u32::MAX);
                assert!(time_left <= max_sleep);

                let time_left: MicroSecond = (time_left).micros();

                // Only sleep
                if time_left.ticks() > 0 {
                    self.0.start(time_left);
                    block!(self.0.wait()).ok();
                }
            }
        }

        impl<T> $Delay<u16> for DelayFromCountDownTimer<T>
            where
                T: CountDown_us<Time = MicroSecond>,
            {
                fn $delay(&mut self, t: u16) {
                    self.$delay(t as u32);
                }
            }

        impl<T> $Delay<u8> for DelayFromCountDownTimer<T>
            where
                T: CountDown_us<Time = MicroSecond>,
            {
                fn $delay(&mut self, t: u8) {
                    self.$delay(t as u32);
                }
            }
    };

    ($Delay:ident, MilliSecond, $delay:ident ) => {
        impl<T> $Delay<u32> for DelayFromCountDownTimer<T>
        where
            T: CountDown_ms<Time = MilliSecond>,
        {
            fn $delay(&mut self, t: u32) {
                let mut time_left = t;

                let max_sleep = self.0.max_period();
                let max_sleep = max_sleep.to_millis();

                if time_left > max_sleep {
                    self.0.start(max_sleep.millis());

                    // Process the time one max_sleep duration at a time
                    // to avoid overflowing both u32 and the timer
                    for _ in 0..(time_left / max_sleep) {
                        block!(self.0.wait()).ok();
                        time_left -= max_sleep;
                    }
                }

                assert!(time_left <= u32::MAX);
                assert!(time_left <= max_sleep);

                let time_left: MilliSecond = (time_left).millis();

                // Only sleep
                if time_left.ticks() > 0 {
                    self.0.start(time_left);
                    block!(self.0.wait()).ok();
                }
            }
        }

        impl<T> $Delay<u16> for DelayFromCountDownTimer<T>
            where
                T: CountDown_ms<Time = MilliSecond>,
            {
                fn $delay(&mut self, t: u16) {
                    self.$delay(t as u32);
                }
            }

            impl<T> $Delay<u8> for DelayFromCountDownTimer<T>
            where
                T: CountDown_ms<Time = MilliSecond>,
            {
                fn $delay(&mut self, t: u8) {
                    self.$delay(t as u32);
                }
            }
    };
}

//If im correct only the nanosecond delay needs to be implemented as the HAL api already converts to ms / us.
impl_delay_from_count_down_timer! {
    DelayNs, NanoSecond, delay_ns
}

//Implementing API 0.2.7
impl_delay_from_count_down_timer_old! {
    DelayUs, MicroSecond, delay_us
}

impl_delay_from_count_down_timer_old! {
    DelayMs, MilliSecond, delay_ms
}
