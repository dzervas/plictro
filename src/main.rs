#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate nb;
extern crate nrf52840_hal;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use nb::block;
use nrf52840_hal as hal;
use hal::gpio::{Level, p0};
use hal::prelude::*;
use hal::pac::Peripherals;
use hal::timer::{Timer, Instance};


#[entry]
fn main() -> ! {

	let periph = Peripherals::take().unwrap();

	// Indicator LED
	let mut timer = Timer::new(periph.TIMER4);
	let p0 = p0::Parts::new(periph.P0);

	let mut led = p0.p0_28.into_push_pull_output(Level::High);

	led.set_low().unwrap();
	let mut is_high = false;

	loop {
		if is_high {
			led.set_low().unwrap();
			is_high = false;
		} else {
			led.set_high().unwrap();
			is_high = true;
		}

		delay(&mut timer, 333_333);
	}

}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
	T: Instance,
{
	timer.start(cycles);
	let _ = block!(timer.wait());
}
