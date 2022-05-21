#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_itm;

extern crate embedded_hal as hal;
extern crate stm32f429i_disc as board;

use cortex_m_rt::entry;

use board::gpio;
use board::gpio::gpiog::{PG13, PG14};
use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;

use cortex_m::iprintln;
use cortex_m::peripheral::Peripherals;

struct Leds {
	green: PG13<gpio::Output<gpio::PushPull>>,
	red: PG14<gpio::Output<gpio::PushPull>>,
}

#[entry]
fn main() -> ! {
	if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
		let gpiog = p.GPIOG.split();
		let mut itm = cp.ITM;

		// Configure LED outputs
		let mut leds = Leds { green: gpiog.pg13.into_push_pull_output(), red: gpiog.pg14.into_push_pull_output() };

		// Constrain clock registers
		let rcc = p.RCC.constrain();

		// Configure clock to 168 MHz (i.e. the maximum) and freeze it
		let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

		// Get delay provider
		let mut delay = Delay::new(cp.SYST, clocks);

		iprintln!(&mut itm.stim[0], "start");

		loop {
			// Turn LED on
			let _ = hal::digital::v2::OutputPin::set_high(&mut leds.green);
			iprintln!(&mut itm.stim[0], "on");

			// Delay twice for half a second due to limited timer resolution
			delay.delay_ms(50_u16);
			// delay.delay_ms(500_u16);
			// delay.delay_ms(500_u16);

			// Turn LED off
			let _ = hal::digital::v2::OutputPin::set_low(&mut leds.green);
			iprintln!(&mut itm.stim[0], "off");

			// Delay twice for half a second due to limited timer resolution
			delay.delay_ms(50_u16);
			// delay.delay_ms(500_u16);
			// delay.delay_ms(500_u16);
			iprintln!(&mut itm.stim[0], "Je suis beaucoup plus rapide que UART et mon cousin Arduino !");
		}
	}

	loop {}
}
