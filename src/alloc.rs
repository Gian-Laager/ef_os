use core::alloc::*;
use core::ptr;

const DEFAULT_PAGE_SIZE: usize = 4096;
const DEFAULT_NUM_PAGES: usize = 1000000 * DEFAULT_PAGE_SIZE / 4;
type PageBuffType<const PAGE_SIZE: usize> = [u8; PAGE_SIZE];
const fn default_page_buffer<const PAGE_SIZE: usize>() -> PageBuffType<PAGE_SIZE> {
    [0u8; PAGE_SIZE]
}

#[derive(Clone, Copy)]
struct Page {
    in_use: bool,
    buff: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            in_use: false,
            buff: usize::max_value(),
        }
    }
}

#[derive(Clone, Copy)]
struct NextFree {
    idx: usize,
    size: usize,
}

impl NextFree {
    fn new(idx: usize, size: usize) -> Self {
        Self { idx, size }
    }
}

struct PageManager<const PAGE_SIZE: usize, const NUM_PAGES: usize> {
    pages: [Page; NUM_PAGES],
    buff: [PageBuffType<PAGE_SIZE>; NUM_PAGES],
    first_free: Option<NextFree>,
}


impl<const PAGE_SIZE: usize, const NUM_PAGES: usize> PageManager<PAGE_SIZE, NUM_PAGES> {
    pub unsafe fn new() -> Self {
        let mut pages = [Page::default(); NUM_PAGES];

        for i in 0..pages.len() {
            pages[i] = Page {
                in_use: false,
                buff: i,
            };
        }

        Self {
            pages,
            buff: [default_page_buffer(); NUM_PAGES],
            first_free: Some(NextFree::new(0, NUM_PAGES)),
        }
    }

    fn get_next_free_idx(&self, search_start: usize) -> Option<usize> {
        for i in search_start..self.pages.len() {
            if !self.pages[i].in_use {
                return Some(i);
            }
        }
        None
    }

    fn get_size_of_free_region(&self, free: usize) -> usize {
        let mut counter = 0;
        let mut i = free;

        while self.pages[i].in_use && i < self.pages.len() {
            i += 1;
            counter += 1;
        }

        return counter;
    }

    fn get_next_free(&self, search_start: usize) -> Option<NextFree> {
        let idx = self.get_next_free_idx(search_start)?;
        let size = self.get_size_of_free_region(idx);
        Some(NextFree::new(idx, size))
    }

    fn mark_as_used(&mut self, begin: usize, size: usize) {
        self.pages[begin..size]
            .iter_mut()
            .for_each(|mut page| page.in_use = true);
    }

    fn find_memory(&mut self, num_pages: usize) -> Option<usize> {
        if self.first_free.is_none() {
            // This should return None but, it's a last check if there's some memory left
            match self.get_next_free(0) {
                None => return None,
                Some(next) => self.first_free = Some(next) 
            }
        }

        if let Some(first_free) = &mut self.first_free {
            if first_free.size < num_pages {
                let result = first_free.idx;
                first_free.idx += num_pages;
                first_free.size -= num_pages;
                return Some(result);
            } else if first_free.size == num_pages {
                let result = first_free.idx;
                let end_of_free = first_free.idx + num_pages;
                self.first_free = self.get_next_free(end_of_free);
                return Some(result);
            } else {
                let end_of_free = first_free.idx + num_pages;
                let mut region = self.get_next_free(end_of_free)?;

                while region.size < num_pages {
                    region = self.get_next_free(region.idx + region.size)?;
                }

                return Some(region.idx);
            }
        }
        return None;
    }

    pub fn allocate(&mut self, num_pages: usize) -> Option<usize> {
        let mem = self.find_memory(num_pages)?;
        self.mark_as_used(mem, num_pages);
        return Some(mem);
    }

    pub fn deallocate(&mut self, idx: )
}

pub struct MemoryAllocator {}

unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        todo!()
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
}
