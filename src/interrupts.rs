use x86_64::structures::idt;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::set_general_handler;
use crate::println;

static mut IDT: idt::InterruptDescriptorTable = idt::InterruptDescriptorTable::new();

pub fn idt_init() {
    unsafe {
        set_general_handler!(&mut IDT, interrupt_test, 255);
        set_general_handler!(&mut IDT, com1_interrupt, 254);
        IDT.page_fault.set_handler_fn(page_fault);
        IDT.load();
    }
}

pub fn interrupt_test(frame: idt::InterruptStackFrame, idx: u8, err: Option<u64>) {
    println!("test interrupt");
}

extern "x86-interrupt" fn page_fault(frame: idt::InterruptStackFrame, err: idt::PageFaultErrorCode) {
    panic!("cpu exception page_fault: {:#?}", err);
}
  
pub fn com1_interrupt(frame: idt::InterruptStackFrame, idx: u8, err: Option<u64>) {
    unsafe {
        let data_ptr = 0x3f8 as *mut usize;
        println!("com1 interrupt data: {}", *data_ptr);
    }
}

