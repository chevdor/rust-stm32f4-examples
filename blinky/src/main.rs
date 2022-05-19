#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;

extern crate cast;
extern crate cortex_m;
extern crate panic_semihosting;
extern crate stm32f4;

use cast::{u16, u32};
use rt::ExceptionFrame;
use stm32f4::stm32f429;

mod frequency {
	/// Frequency of APB1 bus (TIM6 is connected to this bus)
	pub const APB1: u32 = 8_000_000;
}

/// Timer frequency
const FREQUENCY: u32 = 1;

const U16_MAX: u16 = 0xffff;

// On STM32F429ZI-DISC1 board, user LEDS are
//   LD3: green @ PG13
//   LD4: red   @ PG14

#[cortex_m_rt::entry]
fn main() -> ! {
	let peripherals = stm32f429::Peripherals::take().unwrap();

	// Critical section, this closure is non-preemptable
	cortex_m::interrupt::free(|_cs| {
		// INITIALIZATION PHASE

		// Power up the relevant peripherals
		peripherals.RCC.ahb1enr.write(|w| w.gpiogen().set_bit());
		peripherals.RCC.apb1enr.write(|w| w.tim6en().set_bit());

		// Configure the pin PD12 as a pullup output pin
		peripherals.GPIOG.otyper.write(|w| w.ot13().clear_bit());
		peripherals.GPIOG.moder.write(|w| w.moder13().output());
		peripherals.GPIOG.pupdr.write(|w| w.pupdr13().pull_up());

		// Configure TIM6 for periodic timeouts
		let ratio = frequency::APB1 / FREQUENCY;
		let psc = u16((ratio - 1) / u32(U16_MAX)).unwrap();
		let arr = u16(ratio / u32(psc + 1)).unwrap();
		peripherals.TIM6.psc.write(|w| w.psc().bits(psc));
		peripherals.TIM6.arr.write(|w| w.arr().bits(arr));
		peripherals.TIM6.cr1.write(|w| w.opm().clear_bit());

		// Start the timer
		peripherals.TIM6.cr1.modify(|_, w| w.cen().set_bit());

		// APPLICATION LOGIC
		let mut state = false;
		loop {
			// Wait for an update event
			while peripherals.TIM6.sr.read().uif().bit_is_clear() {}

			// Clear the update event flag
			peripherals.TIM6.sr.modify(|_, w| w.uif().clear_bit());

			// Toggle the state
			state = !state;

			// Blink the LED
			peripherals.GPIOG.odr.write(|w| w.odr13().bit(state));
		}
	})
}

// exception!(HardFault, hard_fault);
#[cortex_m_rt::exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
	panic!("HardFault at {:#?}", ef);
}

// exception!(*, default_handler);
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(irqn: i16) {
	panic!("Unhandled exception (IRQn = {})", irqn);
}
