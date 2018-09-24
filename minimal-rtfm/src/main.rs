//! Minimal example with zero tasks
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rtfm as rtfm; // IMPORTANT always do this rename
extern crate stm32f4;
extern crate panic_semihosting;

// import the procedural macro
use rtfm::app;
use stm32f4::stm32f407;

// This macro call indicates that this is a RTFM application
//
// This macro will expand to a `main` function so you don't need to supply
// `main` yourself.
app! {
    // this is the path to the device crate
    device: stm32f407,
}

// The initialization phase.
//
// This runs first and within a *global* critical section. Nothing can preempt
// this function.
fn init(p: init::Peripherals) {
    // This function has access to all the peripherals of the device
    p.core.SYST;
    p.device.GPIOA;
    p.device.RCC;
    // ..
}

// The idle loop.
//
// This runs after `init` and has a priority of 0. All tasks can preempt this
// function. This function can never return so it must contain some sort of
// endless loop.
fn idle() -> ! {
    loop {
        // This puts the processor to sleep until there's a task to service
        rtfm::wfi();
    }
}