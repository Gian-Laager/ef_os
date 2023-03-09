
struct Tty<const MAX_BUFF_SIZE: usize> {
    screen_size: (usize, usize),
    background_char: u8,
    buffer: [u8; MAX_BUFF_SIZE],
}

impl<const MAX_BUFF_SIZE: usize> Tty<MAX_BUFF_SIZE>{
    fn new(screen_size: (usize, usize), background_char: u8) -> Tty<MAX_BUFF_SIZE> {
        return Tty {
            screen_size,
            background_char,
            buffer: [background_char; MAX_BUFF_SIZE]
        }
    }
}
