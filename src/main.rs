#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate nb;
extern crate nrf52840_hal;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use nb::block;
use nrf52840_dk_bsp::Board;
// use nrf52840_dk_bsp::hal as hal;
use nrf52840_dk_bsp::hal::prelude::*;
use nrf52840_dk_bsp::hal::gpio::Level;
use nrf52840_dk_bsp::hal::timer::{self, Timer};

#[entry]
fn main() -> ! {

	let board = Board::take().unwrap();

	let mut timer = Timer::new(board.TIMER4);
	let mut led = board.pins.P0_28.into_push_pull_output(Level::High);
	led.set_high().unwrap();

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
	T: timer::Instance,
{
	timer.start(cycles);
	let _ = block!(timer.wait());
}
