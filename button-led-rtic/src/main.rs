#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
	use fugit::Duration;
	use stm32f4xx_hal::gpio::gpioa::PA0;
	use stm32f4xx_hal::gpio::gpiog::{PG13, PG14};
	use stm32f4xx_hal::gpio::*;
	use stm32f4xx_hal::{
		gpio::{Edge, Input, Output, PushPull},
		prelude::*,
	};
	use systick_monotonic::Systick;

	#[shared]
	struct Shared {}

	#[local]
	struct Local {
		button: PA0<Input<PullDown>>,
		led1: PG13<Output<PushPull>>,
		led2: PG14<Output<PushPull>>,
		state: bool,
	}

	#[monotonic(binds = SysTick, default = true)]
	type MonoTimer = Systick<1000>;

	#[init]
	fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
		let mut syscfg = ctx.device.SYSCFG.constrain();

		let gpiog = ctx.device.GPIOG.split();
		let led1 = gpiog.pg13.into_push_pull_output();
		let led2 = gpiog.pg14.into_push_pull_output();
		let mono = Systick::new(ctx.core.SYST, 36_000_000);

		let gpioa = ctx.device.GPIOA.split();
		let mut button = gpioa.pa0.into_pull_down_input();
		button.make_interrupt_source(&mut syscfg);
		button.enable_interrupt(&mut ctx.device.EXTI);
		button.trigger_on_edge(&mut ctx.device.EXTI, Edge::Rising);
		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(100)).unwrap();
		(Shared {}, Local { button, led1, led2, state: false }, init::Monotonics(mono))
	}

	#[task(local = [led1])]
	fn blink1(cx: blink1::Context) {
		cx.local.led1.toggle();

		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(50)).unwrap();
	}

	#[task(binds = EXTI0, local = [button, state, led2])]
	fn btn_1(cx: btn_1::Context) {
		cx.local.button.clear_interrupt_pending_bit();
		if *cx.local.state {
			cx.local.led2.set_low();
			*cx.local.state = false;
		} else {
			cx.local.led2.set_high();
			*cx.local.state = true;
		}
	}
}
