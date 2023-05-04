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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use core::arch::asm;

    static mut TEST_VAR: bool = false;
    fn interrupt(_frame: idt::InterruptStackFrame, _idx: u8, _err: Option<u64>) {
        unsafe {
            TEST_VAR = true;
        }
    }

    #[os_test]
    fn custom_interrupt() {
        unsafe {
            set_general_handler!(&mut IDT, interrupt, 255);
            software_interrupt!(255);
            assert!(TEST_VAR);
        }
    }
}
