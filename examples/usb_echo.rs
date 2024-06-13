#![no_std]
#![no_main]

use aemics_stm32g4xx_hal as hal;

use hal::preludes::{
    default::*,
    usb::*
};


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

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
        .manufacturer("AEMICS")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        //Poll the USB device. In order to remain compliant with USB standards, this needs to be done at least once every 10 milliseconds.
        usb_dev.poll(&mut [&mut serial]);


        let mut buf = [0u8; 64];

        //Read incoming data
        match serial.read(&mut buf) {
            Ok(_) => {
                // bit veranderen zodat het in hoofdletters terug echo't
                for c in buf[0..64].iter_mut() {
                    if b'a' <= *c && *c <= b'z' {
                        *c &= !0x20;
                    }
                }

                //aangepaste data terugsturen
                serial.write(&buf).unwrap();
            }
            _ => {}
        }
    }
}
