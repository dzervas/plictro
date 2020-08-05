#![no_std]
#![no_main]

extern crate panic_semihosting;

use embedded_hal::timer::Cancel;
use core::fmt::Write;

use cortex_m_rt::entry;
use hal::gpio::{p0, Level};
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::{Instance, Timer};
use hal::twim;
use nb::block;
use nrf52840_hal as hal;

use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyle;
use ssd1306::prelude::*;
use ssd1306::{Builder, I2CDIBuilder};

#[entry]
fn main() -> ! {
	let p = Peripherals::take().unwrap();

	// Indicator LED
	let mut timer = Timer::new(p.TIMER4);
	let port0 = p0::Parts::new(p.P0);

	let mut led = port0.p0_28.into_push_pull_output(Level::High);

	led.set_low().unwrap();
	let mut is_high = false;

	// Serial
	let mut serial = hal::Uarte::new(
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

	writeln!(serial, "Starting Up!").unwrap();

	// Screen
	let scl = port0.p0_27.into_floating_input().degrade();
	let sda = port0.p0_26.into_floating_input().degrade();

	let pins = twim::Pins { scl, sda };

	let i2c = twim::Twim::new(p.TWIM0, pins, twim::Frequency::K400);

	let interface = I2CDIBuilder::new().init(i2c);
	let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

	disp.init().expect("Display initialization");
	disp.flush().expect("Cleans the display");

	let text_style = TextStyle::new(Font6x8, BinaryColor::On);
	let text = "aaaaaaaaaaaaaaaaaaaaaa";
	// let width = text.len() as i32 * 6;

	Text::new(text, Point::new(0, 40))
		.into_styled(text_style)
		.draw(&mut disp)
		.expect("Draw text");

	disp.flush().expect("Render display");

	let mut fps = 0u8;
	timer.start(2_000_000u32); // 1 second

	loop {
		if is_high {
			led.set_low().unwrap();
			is_high = false;
		} else {
			led.set_high().unwrap();
			is_high = true;
		}

		Text::new(text, Point::new(0, 0))
			.into_styled(text_style)
			.draw(&mut disp)
			.unwrap();
		disp.flush().unwrap();

		fps += 1;

		if timer.read() >= 1_000_000u32 {
			writeln!(serial, "FPS: {}", fps).unwrap();
			fps = 0;
			timer.cancel().unwrap();
			timer.start(2_000_000u32); // 1 second
		}
		// timer.

		// delay(&mut timer, 333_333);
	}
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
	T: Instance,
{
	timer.start(cycles);
	let _ = block!(timer.wait());
}
