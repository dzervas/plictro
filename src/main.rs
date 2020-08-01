#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate nb;
extern crate nrf52840_hal;
extern crate panic_semihosting;
extern crate rubble;
extern crate rubble_nrf5x;

// #[allow(unused_imports)]
// use panic_semihosting;

mod hid_service;
pub mod hid_consts;

use cortex_m_rt::entry;
use nb::block;
use nrf52840_dk_bsp::Board;
// use nrf52840_dk_bsp::Led;
use nrf52840_dk_bsp::hal as hal;
use nrf52840_dk_bsp::hal::prelude::*;
use nrf52840_dk_bsp::hal::gpio::Level;
use nrf52840_dk_bsp::hal::timer::{self, Timer};

use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;


#[entry]
fn main() -> ! {

	let board = Board::take().unwrap();

	let mut timer = Timer::new(board.TIMER4);
	let mut led = board.pins.P0_28.into_push_pull_output(Level::High);
	led.set_high().unwrap();

	// ---- Bluetooth

	// On reset, the internal high frequency clock is already used, but we
	// also need to switch to the external HF oscillator. This is needed
	// for Bluetooth to work.
	hal::clocks::Clocks::new(board.CLOCK).enable_ext_hfosc();

	// let mut core = board.
	// hal::timer::Timer::new(board.DCB).

	let device_address = get_device_address();

	// TODO: Fix this abomination
	static mut BLE_TX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
	static mut BLE_RX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
	let mut radio = unsafe { BleRadio::new(
		board.RADIO,
		&board.FICR,
		&mut BLE_TX_BUF,
		&mut BLE_RX_BUF,
	) };

	let beacon = Beacon::new(
		device_address,
		&[AdStructure::CompleteLocalName("Rusty Beacon (nRF52)")],
	).unwrap();

	// ----

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
		beacon.broadcast(&mut radio);

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
