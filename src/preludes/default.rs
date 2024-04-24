//! Module importing the basics for interacting with the HAL.
//! ## Imported modules:
//! - RCC clocks configuration and extention traits
//! - Peripheral access API for stm32g4 MCUs
//! - Low level access to core peripherals of the cortex-m architecture
//! - Startup code and minimal runtime for Cortex-M microcontrollers
//! - panic_semihosting for debugging with GDB.


pub use crate::{
    rcc::{Config, RccExt},
    stm32, //Import peripheral library of the STM32G4 family.
    cortex_m, //Import the core peripherals of the cortex-m architecture. This allows access to the system timer (SYST) for example.
    cortex_m_rt::entry, //Required for targeting a cortex-m platform with Rust code. Handles memory layout, startup, etc.
    panic_semihosting
};


