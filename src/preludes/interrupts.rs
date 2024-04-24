
//Hier importeren we de benodigde functies uit de HAL die we nodig hebben
pub use crate::{
    stm32::{interrupt, Interrupt},
};

pub use core::cell::RefCell;
pub use core::ops::DerefMut;
pub use cortex_m::{asm::wfi, interrupt::Mutex};

//use crate::pwr::PwrExt;