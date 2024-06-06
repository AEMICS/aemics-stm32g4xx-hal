use crate::rcc::{Enable,Reset};
use stm32_usbd::UsbPeripheral;
use stm32_usbd::UsbBus;
use crate::stm32::{USB, RCC};

///A wrapper for the STM32 usb register block.
///
///This is used by the usb-device crates for initializing and driving the internal USB peripheral.
pub struct USBObj {
    /// USB Register Block
    pub usb: USB,
}

unsafe impl Sync for USBObj {}

unsafe impl UsbPeripheral for USBObj
{
    const REGISTERS: *const () = USB::ptr() as *const ();
    const DP_PULL_UP_FEATURE: bool = true;
    const EP_MEMORY: *const () = 0x4000_6000 as _;
    // Endpoint memory size in bytes
    const EP_MEMORY_SIZE: usize = 1_024;

    // Endpoint memory access scheme.
    // Set to `true` if "2x16 bits/word" access scheme is used, otherwise set to `false`.
    const EP_MEMORY_ACCESS_2X16: bool = true;


    fn enable() {
        cortex_m::interrupt::free(|_| unsafe {
            let rcc_ptr = &(*RCC::ptr());
            USB::enable(rcc_ptr);
            USB::reset(rcc_ptr);
        });
    }

    fn startup_delay() {
        cortex_m::asm::delay(80);
    }

}

/// Type of the UsbBus
pub type UsbBusType = UsbBus<USBObj>;

