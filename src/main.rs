#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr};

// The reset handler
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    let mut _x = 0;

    // Register addresses for STM32F411
    const RCC_AHB1ENR: u32 = 0x40023830; // AHB1 peripheral clock enable register
    const GPIOC_MODER: u32 = 0x40020800;  // GPIOC mode register
    const GPIOC_ODR: u32 = 0x40020814;    // GPIOC output data register

    unsafe {
        // Enable clock for GPIOC (bit 2 in RCC_AHB1ENR)
        ptr::write_volatile(RCC_AHB1ENR as *mut u32, ptr::read_volatile(RCC_AHB1ENR as *const u32) | (1 << 2));

        // Set PC13 as output (00 for MODE13[1:0])
        let moder = ptr::read_volatile(GPIOC_MODER as *const u32);
        ptr::write_volatile(GPIOC_MODER as *mut u32, moder & !(0b11 << (13 * 2))); // Reset MODE13
        ptr::write_volatile(GPIOC_MODER as *mut u32, moder | (0b01 << (13 * 2))); // Set MODE13 = 0b01 (output)

        loop {
            plus_one(&mut _x);
            // Turn on PC13 (LED off, as it is connected to VCC)
            ptr::write_volatile(GPIOC_ODR as *mut u32, ptr::read_volatile(GPIOC_ODR as *const u32) & !(1 << 13));
            delay(8_000_000);

            // Turn off PC13 (LED on)
            ptr::write_volatile(GPIOC_ODR as *mut u32, ptr::read_volatile(GPIOC_ODR as *const u32) | (1 << 13));
            delay(8_000_000);
        }
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop") }
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