mod nitro;
mod option;
mod result;

pub use result::Result;
pub use result::ResultExt;

pub use option::OptionExt;

pub use nitro::Nitro;

#[expect(non_snake_case)]
pub fn Ok<T, E: std::error::Error>(value: T) -> Result<T, E, ()> {
    Result::Ok(value)
}

#[expect(non_snake_case)]
pub fn Err<T, E: std::error::Error + 'static>(err: E) -> Result<T, E, ()> {
    Result::Err(Nitro::without_ctx(err))
}
