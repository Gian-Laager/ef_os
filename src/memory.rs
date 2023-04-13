use linked_list_allocator::LockedHeap;

pub const HEAP_SIZE: usize = 1000 * 1024; // 1 MiB

static mut HEAP_BUFF: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .init(core::ptr::addr_of!(HEAP_BUFF) as *mut u8, HEAP_SIZE);
    }
}
