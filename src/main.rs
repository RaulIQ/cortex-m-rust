#![no_std]
#![no_main]

use core::panic::PanicInfo;

// The reset handler
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    let mut _x = 0;

    loop {
        plus_one(&mut _x);
    }
}

fn plus_one(n: &mut u8) {
    *n += 1;
}

// The reset vector, a pointer into the reset handler
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}