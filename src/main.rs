#![no_std]
#![no_main]

pub mod mcp23017;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use hal::gpio::{p0, p1, Level};
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::twim;
pub use nrf52840_hal as hal;

// TODO: Don't do the serial init here, use the mod
// pub mod serial;
// pub use serial::SERIAL;

use mcp23017::Mcp23017;

pub type SafeSerial = Mutex<RefCell<Option<hal::uarte::Uarte<hal::pac::UARTE0>>>>;
pub static SERIAL: SafeSerial = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
	let p = Peripherals::take().unwrap();
	let port0 = p0::Parts::new(p.P0);
	let _port1 = p1::Parts::new(p.P1);

	// Indicator LED
	let mut led = port0.p0_28.into_push_pull_output(Level::High);
	// let mut is_high = true;
	led.set_high().unwrap();

	// Serial
	let serial = hal::Uarte::new(
		p.UARTE0,
		hal::uarte::Pins {
			txd: port0.p0_06.into_push_pull_output(Level::High).degrade(),
			rxd: port0.p0_08.into_floating_input().degrade(),
			cts: None,
			rts: None,
		},
		hal::uarte::Parity::EXCLUDED,
		hal::uarte::Baudrate::BAUD115200,
	);

	free(|cs| {
		SERIAL.borrow(cs).replace(Some(serial));
	});

	// serial::init();

	free(|cs| {
		if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
			writeln!(s, "Initialization complete!").unwrap();
		}
	});

	led.set_low().unwrap();

	// MCP23017

	let scl = port0.p0_27.into_floating_input().degrade();
	let sda = port0.p0_26.into_floating_input().degrade();

	let pins = twim::Pins { scl, sda };

	let mut i2c = twim::Twim::new(p.TWIM0, pins, twim::Frequency::K400);

	let mut expander = Mcp23017::new(&mut i2c, 0);

	expander.borrow(&mut i2c).software_reset().unwrap();
	expander.borrow(&mut i2c).set_bank_a_direction(0).unwrap();
	expander.borrow(&mut i2c).set_bank_a_data(0xFF).unwrap();

	loop {
		// if is_high {
		// 	led.set_low().unwrap();
		// 	is_high = false;
		// } else {
		// 	led.set_high().unwrap();
		// 	is_high = true;
		// }
	}
}

// TODO: What should we do on release? panic-semihosting?
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	free(|cs| {
		if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
			writeln!(s, "{}", info).ok();
		}
	});

	loop {}
}
