#![no_std]
#![no_main]

use core::{panic::PanicInfo, sync::atomic::compiler_fence};

extern crate core;

#[panic_handler]
fn handler(_: &PanicInfo) -> !{
    loop{}
}

#[unsafe(no_mangle)]
fn main() ->! {
    compiler_fence(core::sync::atomic::Ordering::Acquire);
    loop{}
}
