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

use rubble::config::Config;
use rubble::l2cap::{BleChannelMap, L2CAPState};
use rubble::link::{MIN_PDU_BUF, LinkLayer, Responder};
use rubble::link::ad_structure::AdStructure;
use rubble::link::queue::{PacketQueue, SimpleQueue};
use rubble::security::NoSecurity;
use rubble::time::Duration;
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;
use rubble_nrf5x::timer::BleTimer;

pub enum AppConfig {}

impl Config for AppConfig {
	type Timer = BleTimer<hal::pac::TIMER0>;
	type Transmitter = BleRadio;
	type ChannelMapper = BleChannelMap<hid_service::HIDServiceAttrs<'static>, NoSecurity>;
	type PacketQueue = &'static mut SimpleQueue;
}

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

	static mut tx_queue: SimpleQueue = SimpleQueue::new();
	static mut rx_queue: SimpleQueue = SimpleQueue::new();

	// Create TX/RX queues
	let (tx, tx_cons) = unsafe { tx_queue.split() };
	let (rx_prod, rx) = unsafe { rx_queue.split() };

	// Create the actual BLE stack objects
	let ble_timer = BleTimer::init(board.TIMER0);
	let mut ble_ll = LinkLayer::<AppConfig>::new(device_address, ble_timer);

	let ble_r = Responder::<AppConfig>::new(
		tx,
		rx,
		L2CAPState::new(BleChannelMap::with_attributes(hid_service::HIDServiceAttrs::new())),
	);

	// ----

	led.set_low().unwrap();
	let mut is_high = false;

	let next_update = ble_ll.start_advertise(
			Duration::from_millis(200),
			&[AdStructure::CompleteLocalName("Plictro")],
			&mut radio,
			tx_cons,
			rx_prod,
		)
		.unwrap();

	ble_ll.timer().configure_interrupt(next_update);

	loop {
		if is_high {
			led.set_low().unwrap();
			is_high = false;
		} else {
			led.set_high().unwrap();
			is_high = true;
		}
		// beacon.broadcast(&mut radio);

		let bletimer = ble_ll.timer();
		if !bletimer.is_interrupt_pending() {
			continue;
		}
		bletimer.clear_interrupt();

		let cmd = ble_ll.update_timer(&mut radio);
		radio.configure_receiver(cmd.radio);

		ble_ll.timer().configure_interrupt(cmd.next_update);

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
