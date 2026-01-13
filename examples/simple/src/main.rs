use std::fmt::Display;

use nitrogen::Nitro;

#[derive(Debug)]
pub struct Error;
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "first error")
    }
}

#[derive(Debug)]
pub struct NewError(u64);
impl std::error::Error for NewError {}
impl Display for NewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "second error")
    }
}

fn main() {
    let error = Nitro::without_ctx(Error);
    let new_err = error.raise(NewError(67));
    dbg!(new_err);

    println!("Hello, world!");
}
