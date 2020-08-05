use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

use crate::hal;
use hal::gpio::{p0, Level};
use hal::pac::Peripherals;
use hal::uarte::Uarte;

use cortex_m::interrupt::{free, Mutex};

pub static SERIAL: Mutex<RefCell<Option<hal::uarte::Uarte<hal::pac::UARTE0>>>> = Mutex::new(RefCell::new(None));

// pub struct Serial(hal::uarte::Uarte<hal::pac::UARTE0>);

pub fn init() {
	let p = Peripherals::take().unwrap();
	let port0 = p0::Parts::new(p.P0);

	let serial = Uarte::new(
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

	free(|cs| {
		if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
			writeln!(s, "[+] UARTE0 is up").unwrap();
		}
	});
}
