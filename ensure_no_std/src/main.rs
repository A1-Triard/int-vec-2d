#![deny(warnings)]

#![no_std]
#![no_main]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_no_std::exit(99)
}

use core::ffi::{c_char, c_int};
use int_vec_2d::*;

#[no_mangle]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    assert_eq!(Point { x: 1, y: 2}.offset(Vector { x: 5, y: -10 }), Point { x: 6, y: -8 });
    0
}
