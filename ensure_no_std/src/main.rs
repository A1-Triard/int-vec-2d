#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_no_std::exit(99)
}

use int_vec_2d::*;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(Point { x: 1, y: 2}.offset(Vector { x: 5, y: -10 }), Point { x: 6, y: -8 });
    0
}
