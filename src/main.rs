#![no_std]
#![no_main]

pub mod mcp23017;
pub mod hid_service;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;

use cortex_m::asm::delay;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use hal::gpio::{p0, p1, Level};
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::twim;
pub use nrf52840_hal as hal;

use rubble::config::Config;
use rubble::l2cap::{BleChannelMap, L2CAPState};
use rubble::link::{MIN_PDU_BUF, LinkLayer, Responder};
use rubble::link::ad_structure::AdStructure;
use rubble::link::queue::{PacketQueue, SimpleQueue};
use rubble::security::NoSecurity;
use rubble::time::{Timer, Duration};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;
use rubble_nrf5x::timer::BleTimer;

pub enum AppConfig {}

impl Config for AppConfig {
	type Timer = BleTimer<hal::pac::TIMER0>;
	type Transmitter = BleRadio;
	type ChannelMapper = BleChannelMap<hid_service::HIDServiceAttrs, NoSecurity>;
	type PacketQueue = &'static mut SimpleQueue;
}

use mcp23017::Mcp23017;

// TODO: Don't do the serial init here, create a mod
pub type SafeSerial = Mutex<RefCell<Option<hal::uarte::Uarte<hal::pac::UARTE0>>>>;
pub static SERIAL: SafeSerial = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
	let p = Peripherals::take().unwrap();
	let port0 = p0::Parts::new(p.P0);
	let _port1 = p1::Parts::new(p.P1);

	// Indicator LED - A2 on Particle Xenon
	let mut led = port0.p0_28.into_push_pull_output(Level::High);
	let mut is_high = true;
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

	// Bluetooth

	// On reset, the internal high frequency clock is already used, but we
	// also need to switch to the external HF oscillator. This is needed
	// for Bluetooth to work.
	hal::clocks::Clocks::new(p.CLOCK).enable_ext_hfosc();

	// let mut core = board.
	// hal::timer::Timer::new(board.DCB).

	let device_address = get_device_address();

	free(|cs| {
		if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
			writeln!(s, "BT Address: {:?}", &device_address).unwrap();
		}
	});

	// TODO: Fix this abomination
	static mut BLE_TX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
	static mut BLE_RX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
	let mut radio = unsafe { BleRadio::new(
		p.RADIO,
		&p.FICR,
		&mut BLE_TX_BUF,
		&mut BLE_RX_BUF,
	) };

	static mut tx_queue: SimpleQueue = SimpleQueue::new();
	static mut rx_queue: SimpleQueue = SimpleQueue::new();

	// Create TX/RX queues
	let (tx, tx_cons) = unsafe { tx_queue.split() };
	let (rx_prod, rx) = unsafe { rx_queue.split() };

	// Create the actual BLE stack objects
	let ble_timer = BleTimer::init(p.TIMER0);
	let mut ble_ll = LinkLayer::<AppConfig>::new(device_address, ble_timer);

	let mut ble_r = Responder::<AppConfig>::new(
		tx,
		rx,
		L2CAPState::new(BleChannelMap::with_attributes(hid_service::HIDServiceAttrs::new())),
	);

	let next_update = ble_ll
		.start_advertise(
			Duration::from_millis(200),
			&[AdStructure::CompleteLocalName("CONCVRRENS CERTA CELERIS")],
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

		if let Some(cmd) = radio.recv_interrupt(ble_ll.timer().now(), &mut ble_ll) {
			radio.configure_receiver(cmd.radio);
			ble_ll.timer().configure_interrupt(cmd.next_update);

			if cmd.queued_work {
				// If there's any lower-priority work to be done, ensure that happens.
				// If we fail to spawn the task, it's already scheduled.
				// while ble_r.has_work() {
				// 	ble_r.process_one().unwrap();
				// }
			}
		}

		let bletimer = ble_ll.timer();
		if bletimer.is_interrupt_pending() {
			bletimer.clear_interrupt();

			let cmd = ble_ll.update_timer(&mut radio);
			radio.configure_receiver(cmd.radio);

			ble_ll.timer().configure_interrupt(cmd.next_update);

			if cmd.queued_work {
				// If there's any lower-priority work to be done, ensure that happens.
				// If we fail to spawn the task, it's already scheduled.
				// while ble_r.has_work() {
				// 	ble_r.process_one().unwrap();
				// }
			}
		}

		while ble_r.has_work() {
			ble_r.process_one().unwrap();
		}

		// delay(10_000_000);
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
