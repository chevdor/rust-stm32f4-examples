#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate stm32f4xx_hal as board;

use cortex_m::peripheral::Peripherals;
use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4::stm32f429;
use stm32f429i_disc::hal::stm32;
use stm32f4xx_hal::gpio::{gpiog::PG13, Output, PushPull};
use stm32f4xx_hal::prelude::*;
use systick_monotonic::{fugit::Duration, Systick};

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
	use super::*;
	// use crate::app::OutputPin;
	// use embedded_hal::digital::v2::OutputPin;
	// use stm32f429i_disc::gpio::GpioExt;

	#[shared]
	struct Shared {}

	#[local]
	struct Local {
		// led: PG13<Output<PushPull>>,
		state: bool,
	}

	#[monotonic(binds = SysTick, default = true)]
	type MonoTimer = Systick<1000>;

	#[init]
	fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
		let p = stm32::Peripherals::take().unwrap();

		// Setup clocks
		// let mut flash = cx.device.FLASH.constrain();
		let mut rcc = cx.device.RCC.constrain();

		let mono = Systick::new(cx.core.SYST, 36_000_000);

		rtt_init_print!();
		rprintln!("init");

		let mhz = 1_000_000;
		// let _clocks = rcc.cfgr.use_hse(8 * mhz).sysclk(36 * mhz).pclk1(36 * mhz).freeze(&mut flash.acr);

		// Setup LED
		// let gpiog = p.GPIOG.split();
		// let mut led = gpiog.pg13.into_push_pull_output();

		// led.set_high();

		// Schedule the blinking task
		blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
		return (Shared {}, Local { state: false }, init::Monotonics(mono));
	}

	#[task(local = [state])]
	fn blink(cx: blink::Context) {
		rprintln!("blink");
		if *cx.local.state {
			// cx.local.led.set_high();
			*cx.local.state = false;
		} else {
			// cx.local.led.set_low();
			*cx.local.state = true;
		}
		blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
	}
}
