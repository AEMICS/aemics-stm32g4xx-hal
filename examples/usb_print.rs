#![no_std]
#![no_main]

use panic_semihosting as _;
use aemics_stm32g4xx_hal as hal;

use hal::usb::USBObj;
use hal::{
    rcc::{Config, RccExt},
    stm32,
};

use hal::preludes::delay::*;

use cortex_m_rt::entry;
use stm32_usbd::UsbBus;

//USB drivers
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use aemics_stm32g4xx_hal::pwr::PwrExt;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //Enable the HSI48 clock (48MHz) for driving the USB peripheral. This also enables the HSI clock which is used as the SYST clock.
    let pwr = dp.PWR.constrain().freeze();
    let rcc = dp.RCC.freeze(Config::hsi48(), pwr);

    //Create a USB object wrapper which is used for initializing the USB bus.
    let usb = USBObj { usb: dp.USB };
    let usb_bus = UsbBus::new(usb);

    //Create a CDC-ACM SerialPort object, which is used for interacting with the USB peripheral as if it were a UART.
    let mut serial = SerialPort::new(&usb_bus);

    //Create the USB device.
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .product("Serial Port")
        .device_class(USB_CLASS_CDC)
        .build();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        //Poll the USB device. In order to remain compliant with USB standards, this needs to be done at least once every 10 milliseconds.
        usb_dev.poll(&mut [&mut serial]);

        delay.delay_ms(1000);

        serial.write(b"Hello World!\n").unwrap();

    }
}
