use alloc::vec::Vec;
use core::fmt::Write;
use spin;

pub struct VgaOut {
    writing_idx: usize,
    vga_buff: &'static mut [u16],
    screen_size: (usize, usize),
    current_color: u16,
}

impl VgaOut {
    pub fn new(screen_size: (usize, usize), buffer: *mut u16) -> Self {
        let vga_buff =
            unsafe { core::slice::from_raw_parts_mut(buffer, screen_size.0 * screen_size.1) };

        if screen_size.0 == 0 {
            panic!("[VgaOut::init] screen_size width may not be 0.");
        }

        if screen_size.1 == 0 {
            panic!("[VgaOut::init] screen_size height may not be 0.");
        }

        Self {
            writing_idx: 0,
            vga_buff,
            screen_size,
            current_color: 0x0f00,
        }
    }
}

impl VgaOut {
    fn write_char_to_writing_idx(&mut self, c: u8) -> Result<(), core::fmt::Error> {
        match self.vga_buff.get_mut(self.writing_idx) {
            // write the character with the appropriate color
            Some(location) => *location = self.current_color + c as u16,
            None => return Err(core::fmt::Error),
        }
        self.writing_idx += 1;
        Ok(())
    }
}

impl core::fmt::Write for VgaOut {
    fn write_str(&mut self, data: &str) -> Result<(), core::fmt::Error> {
        let mut is_color = false;
        let mut partial_color = 0u16;
        for c in data.as_bytes() {
            if *c == 0x1b {
                // ANSI color escape sequence \033 in string
                is_color = true;
                continue;
            }

            if is_color {
                if *c == b'[' {
                    partial_color = 0;
                    continue;
                } else if *c == b'm' || *c == b';' {
                    // Color is foreground
                    if partial_color >= 30 && partial_color <= 37 {
                        const MASK: u16 = 0b00001111u16 << 8;
                        let vga_color = (partial_color as u16 - 30) << 8;
                        // write to the lower 4 bytes
                        self.current_color = (self.current_color & !MASK) ^ (MASK & vga_color);
                    }

                    // Color is foreground
                    if partial_color >= 90 && partial_color <= 97 {
                        const MASK: u16 = 0b00001111u16 << 8;
                        let vga_color = (partial_color as u16 - 82) << 8;
                        // write to the lower 4 bytes
                        self.current_color = (self.current_color & !MASK) ^ (MASK & vga_color);
                    }

                    // Color is background
                    if partial_color >= 40 && partial_color <= 47 {
                        const MASK: u16 = 0b11110000u16 << 8;
                        let vga_color = (partial_color as u16 - 40) << 12;
                        // write to the higher 4 bytes
                        self.current_color = (self.current_color & !MASK) ^ (MASK & vga_color);
                    }

                    // Color is background
                    if partial_color >= 100 && partial_color <= 107 {
                        const MASK: u16 = 0b11110000u16 << 8;
                        let vga_color = (partial_color as u16 - 92) << 12;
                        // write to the higher 4 bytes
                        self.current_color = (self.current_color & !MASK) ^ (MASK & vga_color);
                    }

                    if partial_color == 0 {
                        self.current_color = 0x0f00;
                    }

                    partial_color = 0;
                }

                if *c >= b'0' && *c <= b'9' {
                    partial_color *= 10;
                    partial_color += *c as u16 - b'0' as u16;
                }

                if *c == b'm' {
                    is_color = false;
                }
            } else {
                if *c == b'\n' {
                    if self.writing_idx % self.screen_size.0 != 0 {
                        let num_spaces =
                            self.screen_size.0 - (self.writing_idx % self.screen_size.0);
                        self.write_str(core::str::from_utf8(&vec![b' '; num_spaces]).unwrap())?;
                    }
                    continue;
                }

                // check if screen needs to scroll
                if self.writing_idx < self.vga_buff.len() {
                    self.write_char_to_writing_idx(*c)?;
                } else {
                    // scroll one row by copying
                    let preserved =
                        Vec::from(&self.vga_buff[self.screen_size.0..(self.vga_buff.len())]);
                    self.vga_buff[0..(self.screen_size.0 * (self.screen_size.1 - 1))]
                        .copy_from_slice(preserved.as_slice());
                    self.writing_idx -= self.screen_size.0;
                    self.write_char_to_writing_idx(*c)?;
                }
            }
        }
        return Ok(());
    }
}

pub const VGA_DEFAULT_SCREEN_SIZE: (usize, usize) = (80, 24);
type VgaOutLock = spin::Mutex<VgaOut>;
lazy_static! {
    static ref VGA_OUT: VgaOutLock =
        spin::Mutex::new(VgaOut::new(VGA_DEFAULT_SCREEN_SIZE, 0xb8000 as *mut u16));
}

pub struct VgaOutRef<'a> {
    pub repr: spin::MutexGuard<'a, VgaOut>,
}

impl<'a> From<spin::MutexGuard<'a, VgaOut>> for VgaOutRef<'a> {
    fn from(guard: spin::MutexGuard<'a, VgaOut>) -> Self {
        Self { repr: guard }
    }
}

impl<'a> core::fmt::Write for VgaOutRef<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.repr.write_str(s)
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::vga::_print(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::vga::_print(core::format_args_nl!($($arg)*));
    }};
}

pub fn vgaout() -> VgaOutRef<'static> {
    (*VGA_OUT).lock().into()
}

pub fn _print(args: core::fmt::Arguments<'_>) {
    vgaout().write_fmt(args).unwrap()
}
