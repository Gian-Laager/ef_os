use x86_64::structures::idt;

pub static mut IDT: idt::InterruptDescriptorTable = idt::InterruptDescriptorTable::new();

pub fn idt_init() {
    unsafe {
        IDT.page_fault.set_handler_fn(page_fault);
        // handlers for all cpu exceptions should be added here
        IDT.load();
    }
}

extern "x86-interrupt" fn page_fault(
    _frame: idt::InterruptStackFrame,
    err: idt::PageFaultErrorCode,
) {
    panic!("cpu exception page_fault: {:#?}", err);
}
