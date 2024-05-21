# STM32G4xx Hardware Abstraction Layer
This project contains the hardware abstraction layer used by the PYglet and PYg Mud Puddle Board Abstraction Layers. It can also be used independently to program MCUs of the STM32G4 family. 

The HAL will implement the [embedded-hal crate](https://docs.rs/embedded-hal/latest/embedded_hal/#embedded-hal) which outlines a generic API to follow for Rust HAL projects.

This HAL uses the [stm32g4 device support crate](https://docs.rs/crate/stm32g4/0.15.1) to interact with the MCU's registers.

Currently, none of the STM32G4 devices are supported. This project aims to support at least the following:
* stm32g473


TODO (Readme): 
* Setup process
* Basic usage
* Link to documentation