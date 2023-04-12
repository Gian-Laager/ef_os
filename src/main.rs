#![no_main]
#![no_std]

#![feature(panic_info_message)]
 
extern crate alloc;

use core::panic::PanicInfo;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

mod memory;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    memory::init_heap();
    
    let mut v = vec![1,2,3];
    v.push(31);

    loop {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
            }
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message: &[u8] = b"################################################################################                                KERNERL  PANICED                                ################################################################################";
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
