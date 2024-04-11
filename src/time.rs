/// This code has been taken from the stm32g0xx-hal project and modified slightly to support
/// STM32G4xx MCUs.
pub use fugit::{
    Duration, ExtU32, HertzU32 as Hertz, HoursDurationU32 as Hour,
    MillisDurationU32 as MilliSecond, MicrosDurationU32 as MicroSecond, MinutesDurationU32 as Minute, NanosDurationU32 as NanoSecond,
    RateExtU32, SecsDurationU32 as Second,
};

/// Baudrate
#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Bps(pub u32);

/// A measurement of a monotonically nondecreasing clock
pub type Instant = fugit::TimerInstantU32<1_000_000>;

/// WeekDay (1-7)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeekDay(pub u32);

/// Date (1-31)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonthDay(pub u32);

/// Week (1-52)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Week(pub u32);

/// Month (1-12)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Month(pub u32);

/// Year
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Year(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub daylight_savings: bool,
}

impl Time {
    pub fn new(hours: Hour, minutes: Minute, seconds: Second, daylight_savings: bool) -> Self {
        Self {
            hours: hours.ticks(),
            minutes: minutes.ticks(),
            seconds: seconds.ticks(),
            daylight_savings,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl Date {
    pub fn new(year: Year, month: Month, day: MonthDay) -> Self {
        Self {
            day: day.0,
            month: month.0,
            year: year.0,
        }
    }
}

pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Day in month
    fn day(self) -> MonthDay;

    /// Month
    fn month(self) -> Month;

    /// Year
    fn year(self) -> Year;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        assert!(self > 0);
        Bps(self)
    }
    fn day(self) -> MonthDay {
        MonthDay(self)
    }

    fn month(self) -> Month {
        Month(self)
    }

    fn year(self) -> Year {
        Year(self)
    }
}

///Method for calculating the duration of a given cycles in NanoSeconds.
pub fn duration_ns(hz: Hertz, cycles: u32) -> NanoSecond {
    let cycles = cycles as u64;
    let clk = hz.raw() as u64;
    let ns = cycles.saturating_mul(1_000_000_000_u64) / clk;
    NanoSecond::from_ticks(ns as u32)
}

///Method for calculating the amount of cycles needed at a given hertz for a given duration in NanoSeconds
pub fn cycles_ns(ns: NanoSecond, clk: Hertz) -> u32 {
    assert!(ns.ticks() > 0);
    let clk = clk.raw() as u64;
    let period = ns.ticks() as u64;
    let cycles = clk.saturating_mul(period) / 1_000_000_000_u64;
    cycles as u32
}

///Method for calculating the duration of a given cycles in MicroSeconds.
pub fn duration_us(hz: Hertz, cycles: u32) -> MicroSecond {
    let cycles = cycles as u64;
    let clk = hz.raw() as u64;
    let ns = cycles.saturating_mul(1_000_000_u64) / clk;
    MicroSecond::from_ticks(ns as u32)
}

///Method for calculating the amount of cycles needed at a given hertz for a given duration in MicroSeconds
pub fn cycles_us(ns: MicroSecond, clk: Hertz) -> u32 {
    assert!(ns.ticks() > 0);
    let clk = clk.raw() as u64;
    let period = ns.ticks() as u64;
    let cycles = clk.saturating_mul(period) / 1_000_000_u64;
    cycles as u32
}

///Method for calculating the duration of a given cycles in MilliSeconds.
pub fn duration_ms(hz: Hertz, cycles: u32) -> MilliSecond {
    let cycles = cycles as u64;
    let clk = hz.raw() as u64;
    let ns = cycles.saturating_mul(1_000_u64) / clk;
    MilliSecond::from_ticks(ns as u32)
}

///Method for calculating the amount of cycles needed at a given hertz for a given duration in MilliSeconds
pub fn cycles_ms(ns: MilliSecond, clk: Hertz) -> u32 {
    assert!(ns.ticks() > 0);
    let clk = clk.raw() as u64;
    let period = ns.ticks() as u64;
    let cycles = clk.saturating_mul(period) / 1_000_u64;
    cycles as u32
}