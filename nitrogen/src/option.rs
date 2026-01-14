use crate::{Nitro, result::Result};
use std::error::Error as StdErrorTrait;

pub trait OptionExt {
    type Some;

    fn ok_or_raise<A, F>(self, err: F) -> Result<Self::Some, A, ()>
    where
        A: StdErrorTrait + 'static,
        F: FnOnce() -> A;
}

impl<T> OptionExt for Option<T> {
    type Some = T;

    #[track_caller]
    fn ok_or_raise<A, F>(self, err: F) -> Result<T, A, ()>
    where
        A: StdErrorTrait + 'static,
        F: FnOnce() -> A,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(Nitro::without_ctx(err())),
        }
    }
}
