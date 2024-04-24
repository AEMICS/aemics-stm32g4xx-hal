
//Import version specific api. (This API changed between embedded-hal v0.2.7 and v1.0.0)
pub use crate::hal_api::digital::*;

pub use crate::{
    gpio::GpioExt,
};