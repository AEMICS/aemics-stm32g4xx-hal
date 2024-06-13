#![no_std]
#![no_main]

//! USB Print example
//!
//! This example shows an interrupt driver USB driver set up to print "Hello World" once every second.
//! It has to be interrupt driven due to the delay used in the program's loop function.
//!
//! Currently the interrupt logic has not been wrapped into a USB driver.

use aemics_stm32g4xx_hal as hal;


use aemics_stm32g4xx_hal::preludes::{
    default::*,
    digital::*,
    interrupts::*,
    timers::*,
    delay::*,
    usb::*
};

//This example uses a mutex with the interrupt routine. This ensures only one process can change the value of our LED at once.
static MUTEX_TIM2: Mutex<RefCell<Option<CountDownTimer<stm32::TIM2>>>> =
    Mutex::new(RefCell::new(None));

static mut MUTEX_USB_SERIAL: Mutex<RefCell<Option<SerialPort<UsbBusType>>>> = Mutex::new(RefCell::new(None));

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;



#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut t2) = MUTEX_TIM2.borrow(cs).borrow_mut().deref_mut()
        {
            usb_interrupt();
            t2.clear_interrupt(Event::TimeOut);
        }
    });
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    //let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut serial) = unsafe { MUTEX_USB_SERIAL.borrow(cs).borrow_mut().deref_mut() }
        {
            if !usb_dev.poll(&mut [serial]) {
                return;
            }
        }
    });


}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //Enable the HSI48 clock (48MHz) for driving the USB peripheral. This also enables the HSI clock which is used as the SYST clock.
    let pwr = dp.PWR.constrain().freeze();
    let mut rcc = dp.RCC.freeze(Config::hsi48(), pwr);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    let usb = USBObj { usb: dp.USB };

    unsafe {
        let bus = UsbBus::new(usb);

        USB_BUS = Some(bus);

        cortex_m::interrupt::free(|cs| MUTEX_USB_SERIAL.borrow(cs).borrow_mut().replace(SerialPort::new(USB_BUS.as_ref().unwrap())));

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("AEMICS")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(USB_CLASS_CDC)
            .build();

        USB_DEVICE = Some(usb_dev);
    }


    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();

    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);

    let mut timer2_i = timer2.start_count_down_us(10.micros());
    timer2_i.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| MUTEX_TIM2.borrow(cs).borrow_mut().replace(timer2_i));

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    loop
    {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut serial) = unsafe { MUTEX_USB_SERIAL.borrow(cs).borrow_mut().deref_mut() }
            {
                serial.write(b"Hello World!\n").unwrap();
            }
        });

        //wfi();
        led.toggle().unwrap();
        delay.delay_ms(1000);
    }
}

