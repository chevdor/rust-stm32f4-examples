#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
	use fugit::Duration;
	use stm32f4xx_hal::gpio::gpiog::{PG13, PG14};
	use stm32f4xx_hal::gpio::*;
	use stm32f4xx_hal::gpio::{Output, PushPull};
	use systick_monotonic::Systick;

	#[shared]
	struct Shared {}

	#[local]
	struct Local {
		led1: PG13<Output<PushPull>>,
		led2: PG14<Output<PushPull>>,
		state: bool,
	}

	#[monotonic(binds = SysTick, default = true)]
	type MonoTimer = Systick<1000>;

	#[init]
	fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
		let gpiog = ctx.device.GPIOG.split();
		let led1 = gpiog.pg13.into_push_pull_output();
		let led2 = gpiog.pg14.into_push_pull_output();
		let mono = Systick::new(ctx.core.SYST, 36_000_000);

		tick::spawn().unwrap();
		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(100)).unwrap();
		(Shared {}, Local { led1, led2, state: true }, init::Monotonics(mono))
	}

	#[task(local = [led1])]
	fn blink1(cx: blink1::Context) {
		cx.local.led1.toggle();
		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(50)).unwrap();
	}

	#[task(local = [led2, state])]
	fn tick(cx: tick::Context) {
		if *cx.local.state {
			cx.local.led2.set_high();
			*cx.local.state = false;
		} else {
			cx.local.led2.set_low();
			*cx.local.state = true;
		}
		tick::spawn_after(Duration::<u64, 1, 1000>::from_ticks(55)).unwrap();
	}
}
