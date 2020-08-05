/*!
A platform agnostic Rust friver for the [mcp23017], based on the
[`embedded-hal`] traits.
## The Device
The [mcp23017] is a GPIO expander that also providers LED driver
and keypad scanning functionality.  The device has two banks
of 8 GPIO pins.
## Supported features
At the time of writing, the support is very bare bones: you can
reset the device and configure basic GPIO functions by operating
on a bank at a time.
## Future
There's scope for some interesting API design choices; for example,
it would be nice to expose the individual GPIO pins as instances
that implement the GPIO traits from [`embedded-hal`] so that they
could in turn be used to interface with other drivers.
The keypad and LED driver functionality has some potential for
nice type safe API design by moving pins/banks into different
modes.
Care needs to be taken to find and ergnomic and zero cost
way to express these APIs.
For the moment we just have a fairly dumb pass through!
## Usage
Import this crate an an `embedded_hal` implementation:
```no_run
extern crate metro_m0 as hal;
extern crate mcp23017;
```
Initialize the I2C bus (differs between the various hal implementations):
```no_run
let mut i2c = hal::I2CMaster3::new(
    &clocks,
    400.khz(),
    peripherals.SERCOM3,
    &mut peripherals.PM,
    &mut peripherals.GCLK,
    // Metro M0 express has I2C on pins PA22, PA23
    Sercom3Pad0::Pa22(pins.pa22.into_function_c(&mut pins.port)),
    Sercom3Pad1::Pa23(pins.pa23.into_function_c(&mut pins.port)),
);
```
and then instantiate the driver:
```no_run
let mut expander = mcp23017::Mcp23017::new(&mut i2c, mcp23017::DEFAULT_ADDRESS);
expander.borrow(&mut i2c).software_reset()?;
expander.borrow(&mut i2c).set_bank_a_direction(0)?;
// read the pins from bank a
let pins = expander.borrow(&mut i2c).get_bank_a_data()?;
```
Note that you must `borrow` the i2c bus for the duration of each operation.
This allows the bus to be shared and used to address other devices in between
operations.
It is possible to take ownership of the bus for a longer period of time.  That
is required for `Future` based usage.  While `Future` based usage isn't supported
at this time, we do have support for this mode of operation:
```no_run
let mut owned = expander.take(i2c);
// Now you can borrow and perform the IO when you are ready:
let pins = owned.borrow().get_bank_a_data()?;
// and relinquish the bus when done
let (expander, i2c) = owned.release();
```
*/
extern crate embedded_hal as hal;
use core::marker::PhantomData;

use hal::blocking::i2c::{Write, WriteRead};

#[derive(Copy, Clone)]
pub enum Register {
	/// I/O direction register A
	IODIRA = 0x00,
	/// Input polarity port register A
	IPOLA = 0x02,
	/// Interrupt-on-change pins A
	GPINTENA = 0x04,
	/// Default value register A
	DEFVALA = 0x06,
	/// Interrupt-on-change control register A
	INTCONA = 0x08,
	/// I/O expander configuration register A
	IOCONA = 0x0A,
	/// GPIO pull-up resistor register A
	GPPUA = 0x0C,
	/// Interrupt flag register A
	INTFA = 0x0E,
	/// Interrupt captured value for port register A
	INTCAPA = 0x10,
	/// General purpose I/O port register A
	GPIOA = 0x12,
	/// Output latch register 0 A
	OLATA = 0x14,

	/// I/O direction register B
	IODIRB = 0x01,
	/// Input polarity port register B
	IPOLB = 0x03,
	/// Interrupt-on-change pins B
	GPINTENB = 0x05,
	/// Default value register B
	DEFVALB = 0x07,
	/// Interrupt-on-change control register B
	INTCONB = 0x09,
	/// I/O expander configuration register B
	IOCONB = 0x0B,
	/// GPIO pull-up resistor register B
	GPPUB = 0x0D,
	/// Interrupt flag register B
	INTFB = 0x0F,
	/// Interrupt captured value for port register B
	INTCAPB = 0x11,
	/// General purpose I/O port register B
	GPIOB = 0x13,
	/// Output latch register 0 B
	OLATB = 0x15,
}

/// The default address of the SparkFun board, with no jumpers
/// applied to alter its address.
pub const DEFAULT_ADDRESS: u8 = 0;

const MCP23017_ADDRESS: u8 = 0x20;

/// The `Mcp23017` struct encapsulates the basic data about the device.
/// You need to `borrow` or `take` the I2C bus to issue IO.
pub struct Mcp23017<I2C>
where
	I2C: WriteRead + Write,
{
	address: u8,
	i2c: PhantomData<I2C>,
}

/// The `Owned` struct encapsulates the device and owns the bus.
/// You need to `borrow` to issue IO, or `release` to release
/// ownership of the bus.
pub struct Owned<I2C>
where
	I2C: WriteRead + Write,
{
	i2c: I2C,
	state: Mcp23017<I2C>,
}

/// The `Borrowed` struct encapsulates the device and a borrowed
/// bus.   This is the struct which holds the actual `impl` for
/// the device driver.
pub struct Borrowed<'a, I2C: 'a>
where
	I2C: WriteRead + Write,
{
	i2c: &'a mut I2C,
	state: &'a mut Mcp23017<I2C>,
}

impl<I2C> Mcp23017<I2C>
where
	I2C: WriteRead + Write,
{
	/// Create an instance.  No implicit initialization is performed.
	/// You will likely want to perform a `software_reset` as the
	/// next step.
	pub fn new(_i2c: &mut I2C, address: u8) -> Self {
		Self {
			i2c: PhantomData,
			address,
		}
	}

	/// Take ownership of the bus and return an `Owned` instance.
	/// This is best suited for asynchronous usage where you initiate IO
	/// and later test to see whether it is complete.  This doesn't make
	/// a lot of sense with the current implementation of the Mcp23017 driver
	/// as no such workflows are supported today.
	/// You probably want the `borrow` method instead.
	pub fn take(self, i2c: I2C) -> Owned<I2C> {
		Owned {
			state: self,
			i2c,
		}
	}

	/// Borrow ownership of the bus and return a `Borrowed` instance that
	/// can be used to perform IO.
	pub fn borrow<'a>(&'a mut self, i2c: &'a mut I2C) -> Borrowed<'a, I2C> {
		Borrowed { state: self, i2c }
	}
}

impl<I2C> Owned<I2C>
where
	I2C: WriteRead + Write,
{
	/// Release ownership of the bus and decompose the `Owned` instance back
	/// into its constituent `Mcp23017` and `I2C` components.
	pub fn release(self) -> (Mcp23017<I2C>, I2C) {
		(self.state, self.i2c)
	}

	/// Create a `Borrowed` instance from this `Owned` instance so that you
	/// can perform IO on it.
	/// Ideally we'd impl `Deref` and `DerefMut` instead of doing this, but
	/// it seems impossible to do this and preserve appropriate lifetimes.
	pub fn borrow(&mut self) -> Borrowed<I2C> {
		Borrowed {
			state: &mut self.state,
			i2c: &mut self.i2c,
		}
	}
}

impl<'a, I2C, E> Borrowed<'a, I2C>
where
	I2C: WriteRead<Error = E> + Write<Error = E> + 'a,
{
	/// Write `value` to `register`
	fn write(&mut self, register: Register, value: u8) -> Result<(), E> {
		self.i2c.write(self.state.address | MCP23017_ADDRESS, &[register as u8, value])
	}

	/// Read a 16-bit value from the register and its successor
	pub fn read_16(&mut self, register: Register) -> Result<u16, E> {
		let mut buf = [0u8; 2];
		self.i2c
			.write_read(self.state.address, &[register as u8], &mut buf)?;
		Ok((buf[0] as u16) << 8 | buf[1] as u16)
	}

	/// Perform a software reset of the module. This restores
	/// the device to its power-on defaults.
	pub fn software_reset(&mut self) -> Result<(), E> {
		// All inputs on port A and B
		self.write(Register::IODIRA, 0xFF)?;
		self.write(Register::IODIRB, 0xFF)
	}

	/// Set the direction for each pin in BankA.
	/// Each 1 bit will be set to output, each 0 bit will
	/// be set to input.
	pub fn set_bank_a_direction(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::IODIRA, mask)
	}

	/// Set the direction for each pin in BankB.
	/// Each 1 bit will be set to output, each 0 bit will
	/// be set to input.
	pub fn set_bank_b_direction(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::IODIRB, mask)
	}

	/// Set the data for each pin in BankA.
	/// Each 1 bit will be set to high, each 0 bit will be set low.
	pub fn set_bank_a_data(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::GPIOA, mask)
	}

	/// Set the data for each pin in BankB.
	/// Each 1 bit will be set to high, each 0 bit will be set low.
	pub fn set_bank_b_data(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::GPIOB, mask)
	}

	/// Get the data for each pin in BankA.
	/// Each 1 bit is set to high, each 0 bit is set low.
	pub fn get_bank_a_data(&mut self) -> Result<u8, E> {
		let mut res = [0u8; 1];
		self.i2c
			.write_read(self.state.address, &[Register::GPIOA as u8], &mut res)?;
		Ok(res[0])
	}

	/// Get the data for each pin in BankB.
	/// Each 1 bit is set to high, each 0 bit is set low.
	pub fn get_bank_b_data(&mut self) -> Result<u8, E> {
		let mut res = [0u8; 1];
		self.i2c
			.write_read(self.state.address, &[Register::GPIOB as u8], &mut res)?;
		Ok(res[0])
	}

	/// Get the data for each pin in BankB and BankA as a 16-bit value.
	/// Each 1 bit is set to high, each 0 bit is set low.
	// TODO: Does this work? GPIOA is 0x09 and B is 0x19
	pub fn get_bank_a_and_b_data(&mut self) -> Result<u16, E> {
		self.read_16(Register::GPIOB)
	}

	/// Set the pull-up for each pin in BankA.
	/// Each 1 bit will have pull up enabled, 0 disabled
	pub fn set_bank_a_pullup(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::GPPUA, mask)
	}

	/// Set the pull-up for each pin in BankB.
	/// Each 1 bit will have pull up enabled, 0 disabled
	pub fn set_bank_b_pullup(&mut self, mask: u8) -> Result<(), E> {
		self.write(Register::GPPUB, mask)
	}
}
