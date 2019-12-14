use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Error<T: fmt::Debug> {
    err: T,
}

impl<T: fmt::Debug> Error<T> {
    pub fn boxed(err: T) -> Box<Self> {
        Box::new(Error { err })
    }
}

impl<T: fmt::Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self.err)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {}
