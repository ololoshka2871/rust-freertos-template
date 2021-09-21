#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use freertos_rust::*;
use core::alloc::Layout;

use cortex_m;

extern crate panic_halt; // panic handler

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;


#[entry]
fn main() -> ! {
    Task::new().name("thread").stack_size(128).priority(TaskPriority(2)).start(move || {
        loop{
        }
    }).unwrap();
    FreeRtosUtils::start_scheduler();
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    panic!("Unregistred irq: {}", irqn);
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    //set_led(true);
    asm::bkpt();
    loop {}
}

#[no_mangle]
fn vApplicationStackOverflowHook(_pxTask: FreeRtosTaskHandle, _pcTaskName: FreeRtosCharPtr) {
    asm::bkpt();
}