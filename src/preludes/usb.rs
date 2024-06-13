pub use crate::{
    usb::USBObj,
    stm32_usbd::UsbBus,
    usb_device::prelude::*,
    usbd_serial::{SerialPort, USB_CLASS_CDC},
    usb_device::bus::UsbBusAllocator,
    usb::UsbBusType,
};