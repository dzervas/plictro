#![no_std]
#![no_main]

// extern crate panic_semihosting;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use hal::gpio::{p0, p1, Level};
use hal::pac::Peripherals;
use hal::prelude::*;
pub use nrf52840_hal as hal;

// TODO: Don't do the serial init here, use the mod
// pub mod serial;
// pub use serial::SERIAL;

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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	free(|cs| {
		if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
			writeln!(s, "{}", info).ok();
		}
	});

	loop {}
}
