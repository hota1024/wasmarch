#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidIndexForType(usize),
    InvalidIndexForFuncType(usize),
    InvalidIndexForCode(usize),
}

pub type Result<T> = std::result::Result<T, Error>;
