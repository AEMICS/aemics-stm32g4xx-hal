//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).

use crate::delay::{CountDown_ns, CountDown_us, CountDown_ms, CountDownCompat};
use cast::{u16, u32};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::{DCB, DWT, SYST};
use hal_api_old::timer::{CountDown};
use hal_api_custom::timer::{CountDownNs, CountDownUs, CountDownMs};
use void::Void;

use crate::stm32::RCC;

use crate::rcc::{self, Clocks};
use crate::time::{Hertz, NanoSecond, MicroSecond, MilliSecond};

/// Timer wrapper
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

/// Hardware timers
pub struct CountDownTimer<TIM> {
    tim: TIM,
    clk: Hertz,
}

mod old_api_compat {
    use crate::timer::*;

    impl<TIM> Timer<TIM>
        where
            CountDownTimer<TIM>: CountDownCompat<Time = MicroSecond>,
    {
        /// Starts timer in count down mode at a given frequency
        /// This is a compatibility function for using embedded-HAL 0.2.7.
        /// For new projects instead use start_count_down_ms/us/ns.
        pub fn start_count_down_compat<T>(self, timeout: T) -> CountDownTimer<TIM>
            where
                T: Into<MicroSecond>,
        {
            let Self { tim, clk } = self;
            let mut timer = CountDownTimer { tim, clk };
            timer.start(timeout);
            timer
        }
    }

    impl<TIM> hal_api_old::timer::Periodic for CountDownTimer<TIM> {}
}


impl<TIM> Timer<TIM>
    where
        CountDownTimer<TIM>: CountDown_ns<Time = NanoSecond>,
{
    /// Starts timer in count down mode at a given frequency
    pub fn start_count_down_ns<T>(self, timeout: T) -> CountDownTimer<TIM>
        where
            T: Into<NanoSecond>,
    {
        let Self { tim, clk } = self;
        let mut timer = CountDownTimer { tim, clk };
        timer.start(timeout);
        timer
    }
}

impl<TIM> Timer<TIM>
    where
        CountDownTimer<TIM>: CountDown_us<Time = MicroSecond>,
{
    /// Starts timer in count down mode at a given frequency
    pub fn start_count_down_us<T>(self, timeout: T) -> CountDownTimer<TIM>
        where
            T: Into<MicroSecond>,
    {
        let Self { tim, clk } = self;
        let mut timer = CountDownTimer { tim, clk };
        timer.start(timeout);
        timer
    }
}

impl<TIM> Timer<TIM>
    where
        CountDownTimer<TIM>: CountDown_ms<Time = MilliSecond>,
{
    /// Starts timer in count down mode at a given frequency
    pub fn start_count_down_ms<T>(self, timeout: T) -> CountDownTimer<TIM>
        where
            T: Into<MilliSecond>,
    {
        let Self { tim, clk } = self;
        let mut timer = CountDownTimer { tim, clk };
        timer.start(timeout);
        timer
    }
}

impl<TIM> hal_api_custom::timer::Periodic for CountDownTimer<TIM> {}

/// Interrupt events
pub enum Event {
    /// CountDownTimer timed out / count down ended
    TimeOut,
}

/// Trigger output source
pub enum TriggerSource {
    /// Timer reset - UG as trigger output
    Reset,
    /// Timer enable - CNT_EN as trigger output
    Enable = 0b001,
    /// Update event - Update event as trigger output
    Update = 0b010,
    /// Compare Pulse - Positive pulse if CC1IF is setted
    ComparePulse = 0b011,
    /// Compare1 - OC1REFC as trigger output
    Compare1 = 0b100,
    /// Compare2 - OC2REFC as trigger output
    Compare2 = 0b101,
    /// Compare3 - OC3REFC as trigger output
    Compare3 = 0b110,
    /// Compare4 - OC4REFC as trigger output
    Compare4 = 0b111,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// CountDownTimer is disabled
    Disabled,
}

impl Timer<SYST> {
    /// Initialize timer
    pub fn syst(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        Self {
            tim: syst,
            clk: clocks.sys_clk,
        }
    }

    pub fn release(self) -> SYST {
        self.tim
    }
}

impl CountDownTimer<SYST> {
    /// Starts listening for an `event`
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.disable_interrupt(),
        }
    }
}

trait SharedWait {
    fn shared_wait(&mut self) -> nb::Result<(), Void>;
}

impl SharedWait for CountDownTimer<SYST> {
    fn shared_wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl hal_api_old::timer::CountDown for CountDownTimer<SYST> {
    type Time = MicroSecond;

    fn start<T>(&mut self, timeout: T)
        where
            T: Into<MicroSecond>,
    {
        let rvr = crate::time::cycles_us(timeout.into(), self.clk) - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        self.shared_wait()
    }
}

impl hal_api_custom::timer::CountDown for CountDownTimer<SYST> {
    fn wait(&mut self) -> nb::Result<(), Void> {
        self.shared_wait()
    }
}

impl hal_api_custom::timer::CountDownNs for CountDownTimer<SYST> {
    type Time = NanoSecond;

    fn start<T>(&mut self, timeout: T)
        where
            T: Into<NanoSecond>,
    {
        let rvr = crate::time::cycles_ns(timeout.into(), self.clk) - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }
}

impl hal_api_custom::timer::CountDownUs for CountDownTimer<SYST> {
    type Time = MicroSecond;

    fn start<T>(&mut self, timeout: T)
        where
            T: Into<MicroSecond>,
    {
        let rvr = crate::time::cycles_us(timeout.into(), self.clk) - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }
}

impl hal_api_custom::timer::CountDownMs for CountDownTimer<SYST> {
    type Time = MilliSecond;

    fn start<T>(&mut self, timeout: T)
        where
            T: Into<MilliSecond>,
    {
        let rvr = crate::time::cycles_ms(timeout.into(), self.clk) - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }
}

impl CountDownCompat for CountDownTimer<SYST> {
    fn max_period(&self) -> MicroSecond {
        crate::time::duration_us(self.clk, (1 << 24) - 1)
    }
}

impl CountDown_ns for CountDownTimer<SYST> {
    fn max_period(&self) -> NanoSecond { crate::time::duration_ns(self.clk, (1 << 24) - 1) }
}

impl CountDown_us for CountDownTimer<SYST> {
    fn max_period(&self) -> MicroSecond { crate::time::duration_us(self.clk, (1 << 24) - 1) }
}

impl CountDown_ms for CountDownTimer<SYST> {
    fn max_period(&self) -> MilliSecond { crate::time::duration_ms(self.clk, (1 << 24) - 1) }
}

trait SharedCancel
{
    type Error;
    fn shared_cancel(&mut self) -> Result<(), Self::Error>;
}

impl SharedCancel for CountDownTimer<SYST> {
    type Error = Error;
    fn shared_cancel(&mut self) -> Result<(), Self::Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Self::Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl hal_api_old::timer::Cancel for CountDownTimer<SYST> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.shared_cancel()
    }
}

impl hal_api_custom::timer::Cancel for CountDownTimer<SYST> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.shared_cancel()
    }
}



/// A monotonic non-decreasing timer
///
/// This uses the timer in the debug watch trace peripheral. This means, that if the
/// core is stopped, the timer does not count up. This may be relevant if you are using
/// cortex_m_semihosting::hprintln for debugging in which case the timer will be stopped
/// while printing
#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: Hertz,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, mut dcb: DCB, clocks: &Clocks) -> Self {
        dcb.enable_trace();
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or reset
        #[allow(clippy::drop_non_drop)]
        drop(dwt);

        MonoTimer {
            frequency: clocks.ahb_clk,
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(self) -> Instant {
        Instant {
            now: DWT::cycle_count(),
        }
    }
}

/// A measurement of a monotonically non-decreasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(self) -> u32 {
        DWT::cycle_count().wrapping_sub(self.now)
    }
}

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + rcc::GetBusFreq {}

impl<TIM> Timer<TIM>
    where
        TIM: Instance,
{
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            TIM::enable(rcc);
            TIM::reset(rcc);
        }

        Self {
            clk: TIM::get_timer_frequency(clocks),
            tim,
        }
    }
}

macro_rules! hal_ext_trgo {
    ($($TIM:ty: ($tim:ident, $mms:ident),)+) => {
        $(
            impl Timer<$TIM> {
                pub fn set_trigger_source(&mut self, trigger_source: TriggerSource) {
                    self.tim.cr2.modify(|_, w| unsafe {w.$mms().bits(trigger_source as u8)});
                }
            }
        )+
    }
}

macro_rules! start_func {
    (NanoSecond) => {
        fn start<T>(&mut self, timeout: T)
            where
                T: Into<NanoSecond>,
            {
                // pause
                self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                // reset counter
                self.tim.cnt.reset();

                let ticks = crate::time::cycles_ns(timeout.into(), self.clk);

                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                self.tim.psc.write(|w| unsafe {w.psc().bits(psc)} );

                // TODO: TIM2 and TIM5 are 32 bit
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                self.tim.cr1.modify(|_, w| w.urs().set_bit());
                self.tim.egr.write(|w| w.ug().set_bit());
                self.tim.cr1.modify(|_, w| w.urs().clear_bit());

                // start counter
                self.tim.cr1.modify(|_, w| w.cen().set_bit());
            }
    };

    (MicroSecond) => {
        fn start<T>(&mut self, timeout: T)
            where
                T: Into<MicroSecond>,
            {
                // pause
                self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                // reset counter
                self.tim.cnt.reset();

                let ticks = crate::time::cycles_us(timeout.into(), self.clk);

                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                self.tim.psc.write(|w| unsafe {w.psc().bits(psc)} );

                // TODO: TIM2 and TIM5 are 32 bit
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                self.tim.cr1.modify(|_, w| w.urs().set_bit());
                self.tim.egr.write(|w| w.ug().set_bit());
                self.tim.cr1.modify(|_, w| w.urs().clear_bit());

                // start counter
                self.tim.cr1.modify(|_, w| w.cen().set_bit());
            }
    };

    (MilliSecond) => {
        fn start<T>(&mut self, timeout: T)
            where
                T: Into<MilliSecond>,
            {
                // pause
                self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                // reset counter
                self.tim.cnt.reset();

                let ticks = crate::time::cycles_ms(timeout.into(), self.clk);

                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                self.tim.psc.write(|w| unsafe {w.psc().bits(psc)} );

                // TODO: TIM2 and TIM5 are 32 bit
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                self.tim.cr1.modify(|_, w| w.urs().set_bit());
                self.tim.egr.write(|w| w.ug().set_bit());
                self.tim.cr1.modify(|_, w| w.urs().clear_bit());

                // start counter
                self.tim.cr1.modify(|_, w| w.cen().set_bit());
            }
    };
}

macro_rules! hal {
    ($($TIM:ty: ($tim:ident),)+) => {
        $(
            impl Instance for $TIM { }

            impl CountDownTimer<$TIM> {
                /// Starts listening for an `event`
                ///
                /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
                /// receiving events.
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().set_bit());
                        }
                    }
                }

                /// Clears interrupt associated with `event`.
                ///
                /// If the interrupt is not cleared, it will immediately retrigger after
                /// the ISR has finished.
                pub fn clear_interrupt(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Clear interrupt flag
                            self.tim.sr.write(|w| w.uif().clear_bit());
                        }
                    }
                }

                /// Stops listening for an `event`
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().clear_bit());
                        }
                    }
                }

                /// Releases the TIM peripheral
                pub fn release(self) -> $TIM {
                    // pause counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }

            impl SharedWait for CountDownTimer<$TIM> {
                fn shared_wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl hal_api_old::timer::CountDown for CountDownTimer<$TIM> {
                type Time = MicroSecond;

                start_func!(MicroSecond);

                fn wait(&mut self) -> nb::Result<(), Void> {
                    self.shared_wait()
                }
            }

            impl hal_api_custom::timer::CountDown for CountDownTimer<$TIM> {

                fn wait(&mut self) -> nb::Result<(), Void> {
                     self.shared_wait()
                }
            }

            impl hal_api_custom::timer::CountDownNs for CountDownTimer<$TIM> {

                type Time = NanoSecond;

                start_func!(NanoSecond);
            }

            impl hal_api_custom::timer::CountDownUs for CountDownTimer<$TIM> {

                type Time = MicroSecond;

                start_func!(MicroSecond);
            }

            impl hal_api_custom::timer::CountDownMs for CountDownTimer<$TIM> {

                type Time = MilliSecond;

                start_func!(MilliSecond);
            }

            // TODO: TIM2 and TIM5 are 32 bit
            impl CountDownCompat for CountDownTimer<$TIM> {
                fn max_period(&self) -> MicroSecond {
                    crate::time::duration_us(self.clk, u16::MAX as u32)
                }
            }

            impl CountDown_ns for CountDownTimer<$TIM> {
                fn max_period(&self) -> NanoSecond {
                    crate::time::duration_ns(self.clk, u16::MAX as u32)
                }
            }

            impl CountDown_us for CountDownTimer<$TIM> {
                fn max_period(&self) -> MicroSecond {
                    crate::time::duration_us(self.clk, u16::MAX as u32)
                }
            }

            impl CountDown_ms for CountDownTimer<$TIM> {
                fn max_period(&self) -> MilliSecond {
                    crate::time::duration_ms(self.clk, u16::MAX as u32)
                }
            }

            impl SharedCancel for CountDownTimer<$TIM> {
                type Error = Error;
                fn shared_cancel(&mut self) -> Result<(), Self::Error> {
                   // let is_counter_enabled = self.tim.cr1.read().cen().is_enabled();
                    let is_counter_enabled = self.tim.cr1.read().cen().bit_is_set();
                    if !is_counter_enabled {
                        return Err(Self::Error::Disabled);
                    }

                    // disable counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    Ok(())
                }
            }

            impl hal_api_old::timer::Cancel for CountDownTimer<$TIM>
            {
                type Error = Error;

                fn cancel(&mut self) -> Result<(), Self::Error> {
                     self.shared_cancel()
                }
            }

            impl hal_api_custom::timer::Cancel for CountDownTimer<$TIM>
            {
                type Error = Error;

                fn cancel(&mut self) -> Result<(), Self::Error> {
                     self.shared_cancel()
                }
            }
        )+
    }
}

hal! {
    crate::stm32::TIM1: (tim1),
    crate::stm32::TIM2: (tim2),
    crate::stm32::TIM3: (tim3),
    crate::stm32::TIM4: (tim4),
    crate::stm32::TIM6: (tim6),
    crate::stm32::TIM7: (tim7),
    crate::stm32::TIM8: (tim8),

    crate::stm32::TIM15: (tim15),
    crate::stm32::TIM16: (tim16),
    crate::stm32::TIM17: (tim17),
}

hal_ext_trgo! {
    crate::stm32::TIM1: (tim1, mms2),
    crate::stm32::TIM2: (tim2, mms2),
    crate::stm32::TIM3: (tim3, mms2),
    crate::stm32::TIM4: (tim4, mms2),
    crate::stm32::TIM6: (tim6, mms),
    crate::stm32::TIM7: (tim7, mms),
    crate::stm32::TIM8: (tim8, mms2),

    crate::stm32::TIM15: (tim15, mms),
}

#[cfg(any(
feature = "stm32g471",
feature = "stm32g473",
feature = "stm32g474",
feature = "stm32g483",
feature = "stm32g484"
))]
hal! {
    crate::stm32::TIM5: (tim5),
}

#[cfg(any(
feature = "stm32g473",
feature = "stm32g474",
feature = "stm32g483",
feature = "stm32g484"
))]
hal! {
    crate::stm32::TIM20: (tim20),
}
