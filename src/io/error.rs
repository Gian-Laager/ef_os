use alloc::string::String;

pub enum ErrorKind {
}

pub struct Error {
    message: String,
    kind: ErrorKind
}


pub type Result<T> = core::result::Result<T, Error>;

