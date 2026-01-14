use crate::Nitro;
use std::error::Error as StdErrorTrait;

pub type Result<T, E, CTX> = std::result::Result<T, Nitro<E, CTX>>;

pub trait ResultExt {
    type Success;
    type Error: StdErrorTrait;
    type CTX;

    fn or_raise<Err, F>(self, err: F) -> Result<Self::Success, Err, Self::CTX>
    where
        Err: StdErrorTrait + 'static,
        F: FnOnce() -> Err;
}

impl<T, E> ResultExt for std::result::Result<T, E>
where
    E: StdErrorTrait + 'static,
{
    type Success = T;
    type Error = E;
    type CTX = ();

    #[track_caller]
    fn or_raise<Err, F>(self, err: F) -> Result<Self::Success, Err, ()>
    where
        Err: StdErrorTrait + 'static,
        F: FnOnce() -> Err,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Nitro::without_ctx(e).raise(err())),
        }
    }
}

impl<T, E, CTX> ResultExt for std::result::Result<T, Nitro<E, CTX>>
where
    E: StdErrorTrait + 'static,
{
    type Success = T;
    type Error = E;
    type CTX = CTX;

    #[track_caller]
    fn or_raise<Err, F>(self, err: F) -> Result<Self::Success, Err, CTX>
    where
        Err: StdErrorTrait + 'static,
        F: FnOnce() -> Err,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.raise(err())),
        }
    }
}
