#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(custom_test_frameworks)]
#![feature(asm_const)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate macros;


#[macro_use]
extern crate x86_64;

use core::panic::PanicInfo;
use core::arch::asm;
use x86_64::software_interrupt;

mod memory;
#[cfg(test)]
mod test;
mod vga;
mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    memory::init_heap();
    interrupts::idt_init();

    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();

    loop {}
}

fn main() {
    println!("main");
    let com_ports = 0x3f8 as *mut usize;
    unsafe {
        match com_ports.as_ref() {
            Some(value) => println!("value: {}", value.clone()),
            None => println!("pointer error")
        }
    }
    // unsafe {
    //     let com_ports = 0x3f8 as *mut usize;
    //        
    //     *com_ports.offset(1) = 1;
    //     *com_ports.offset(2) = 254;
    // }
}


static mut PANIC_COUNT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// This function is called on panic.
#[cfg_attr(not(test), panic_handler)]
fn print_panic_banner(info: &PanicInfo) -> ! {
    let panic_count = unsafe { PANIC_COUNT.load(core::sync::atomic::Ordering::Acquire) };
    unsafe { PANIC_COUNT.store(panic_count + 1, core::sync::atomic::Ordering::Release) };

    if panic_count > 2 {
        // panic code paniced 3 times, fail safe to enter an infinite loop without any output.
        loop {}
    }

    if panic_count > 0 {
        // panic code paniced before, fall back to manually overriding vga buffer
        let vga_buffer = 0xb8000 as *mut u8;
        let message: &[u8] = b"################################################################################                                 KERNEL PANICED                                 ################################################################################";
        for (i, &byte) in message.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = 0x4F;
            }
        }

        // clear rest of screen
        for i in vga::VGA_DEFAULT_SCREEN_SIZE.0 * 3
            ..vga::VGA_DEFAULT_SCREEN_SIZE.0 * vga::VGA_DEFAULT_SCREEN_SIZE.1
        {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = 0;
                *vga_buffer.offset(i as isize * 2 + 1) = 0;
            }
        }

        if panic_count == 1 {
            // print panic message
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
        }
    } else {
        println!();
        println!("################################################################################\n                                 KERNEL PANICED                                 \n################################################################################");
        println!("{}", info);
    }

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic_handler(info: &PanicInfo) -> ! {
    let panic_count = unsafe { PANIC_COUNT.load(core::sync::atomic::Ordering::Acquire) };
    unsafe { PANIC_COUNT.store(panic_count + 1, core::sync::atomic::Ordering::Release) };

    // fall back to default handler
    if panic_count > 0 {
        unsafe { PANIC_COUNT.store(1, core::sync::atomic::Ordering::Release) };
        print_panic_banner(info);
    }

    println!();
    println!("################################# TEST  FAILED #################################");
    println!("{}", info);
    loop {}
}
