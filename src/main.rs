#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate core;

#[panic_handler]
fn handler(_: &PanicInfo) -> !{
    loop{}
}

#[unsafe(no_mangle)]
fn main() ->! {
    loop{}
}
