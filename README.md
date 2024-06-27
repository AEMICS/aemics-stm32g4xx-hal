# STM32G4xx Hardware Abstraction Layer
This project contains the hardware abstraction layer used by the PYglet and PYg Mud Puddle Board Abstraction Layers. It can also be used independently to program MCUs of the STM32G4 family. 

The HAL implements the [embedded-hal crate](https://docs.rs/embedded-hal/latest/embedded_hal/#embedded-hal) which outlines a generic API to follow for Rust HAL projects.

This HAL uses the [stm32g4 device support crate](https://docs.rs/crate/stm32g4/0.15.1) to interact with the MCU's registers.

Currently, the only verified supported device is the STM32G473.

## Setup
### prerequisites:
	1. WINDOWS:  C++ build tools for Visual Studio 2019.
	

### install rustup
	go to https://www.rust-lang.org/tools/install
	download and run installer.
	select standard installation
	check if installed by running rustc -V
		Should say "rustc 1.78.0" or greater.

	Note: 	In case of an existing installation, try running: rustup update
	

### set up tooling 
	run rustup target add thumbv7em-none-eabihf
	cargo install cargo-binutils 
	rustup component add llvm-tools

## set up stm32cubeprog
	https://www.st.com/en/development-tools/stm32cubeprog.html
	run installer, standard settings. Make a note of the installation folder.
	Add the installation's 'bin' folder to your PATH environment variable.
	Restart your pc

	Note: 	At time of writing, the stm32cubeprogrammer installer does not support Wayland display driver.
		If this issue comes up, instead try using dfu-util.


## Usage
To add this HAL to your project, add the following line to your project's Cargo.toml file:

```
[dependencies]
aemics-stm32g4xx-hal = { git = "https://gitlab.aemics.nl/aepym/30023200.git", features = ["rt","stm32g473"]}
```

It is also possible to use the crates.io name, which when set up (to do as of 27-06-2024) should look like:

```
[dependencies]
aemics-stm32g4xx-hal = "0.1.0" 
```

The /examples/ folder contains many examples which set up simple programs such as a blinky test program or I2C communication to use when getting started.


## Version History
* **20-03-2024: Version 0.0.0**
    Initial version of the project, implementing only embedded-hal v0.2.7. 
* **27-06-2024: 0.1.0**
    Implementing both embedded-hal v1.0.0 and v0.2.7. 
    USB support added
