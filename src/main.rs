#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(format_args_nl)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

mod memory;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    memory::init_heap();

    for i in 0..40 {
        println!("println test: hello {}", i);
    }
    
    loop {
    }
}

/// This function is called on panic.
#[panic_handler]
fn print_panic_banner(info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message: &[u8] = b"################################################################################                                 KERNEL PANICED                                 ################################################################################";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x4F;
        }
    }

    if let Some(msg) = info.message() {
        if let Some(msg_str) = msg.as_str() {
            for (i, &byte) in msg_str.as_bytes().iter().enumerate() {
                unsafe {
                    *vga_buffer.offset((i + message.len()) as isize * 2) = byte;
                    *vga_buffer.offset((i + message.len()) as isize * 2 + 1) = 0x0F;
                }
            }
        }
    }

    loop {}
}
