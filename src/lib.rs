#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(any(
    feature = "stm32g431",
    feature = "stm32g441",
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484",
    feature = "stm32g491",
    feature = "stm32g4a1"
)))]

compile_error!(
    "This crate requires one of the following features enabled:
        stm32g431
        stm32g441
        stm32g473
        stm32g474
        stm32g483
        stm32g484
        stm32g491
        stm32g4a1"
);

extern crate bare_metal;
extern crate void;

pub extern crate cortex_m;
pub extern crate embedded_hal_1 as hal_api;
pub extern crate embedded_hal_02 as hal_api_old;
pub extern crate nb;
pub extern crate stm32g4;

//Explicitly add these external crates to the HAL crate so projects implementing this HAL can use these two crates.
pub extern crate cortex_m_rt;
pub extern crate panic_semihosting;

pub extern crate stm32_usbd;
pub extern crate usb_device;
pub extern crate usbd_serial;

pub use nb::block;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

#[cfg(feature = "stm32g431")]
pub use stm32g4::stm32g431 as stm32;

#[cfg(feature = "stm32g441")]
pub use stm32g4::stm32g441 as stm32;

#[cfg(feature = "stm32g471")]
pub use stm32g4::stm32g471 as stm32;

#[cfg(feature = "stm32g473")]
pub use stm32g4::stm32g473 as stm32;

#[cfg(feature = "stm32g474")]
pub use stm32g4::stm32g474 as stm32;

#[cfg(feature = "stm32g483")]
pub use stm32g4::stm32g483 as stm32;

#[cfg(feature = "stm32g484")]
pub use stm32g4::stm32g484 as stm32;

#[cfg(feature = "stm32g491")]
pub use stm32g4::stm32g491 as stm32;

#[cfg(feature = "stm32g4a1")]
pub use stm32g4::stm32g4a1 as stm32;

#[cfg(feature = "rt")]
pub use crate::stm32::interrupt;

pub mod adc;
pub mod bb;
pub mod can;
pub mod comparator;
// pub mod crc;
pub mod dac;
pub mod delay;
pub mod dma;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod i2c_compat;
pub mod opamp;
pub mod prelude;
pub mod pwm;
pub mod pwr;
// pub mod qei;
pub mod rcc;
// pub mod rng;
pub mod serial;
pub mod signature;
pub mod spi_compat;
// pub mod stopwatch;
pub mod syscfg;
pub mod time;
pub mod timer;
// pub mod watchdog;
pub mod independent_watchdog;
pub mod i2c;
pub mod preludes;
pub mod spi;
pub mod usb;
pub mod hal_api_custom;
