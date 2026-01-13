use core::fmt;
use std::error::Error as StdErrorTrait;
use std::fmt::Display;
use std::marker::PhantomData;
use std::panic::Location;

pub struct Nitro<Err: StdErrorTrait, CTX = ()> {
    inner: Box<NitroInner<Err, CTX>>,
}

impl<Err: StdErrorTrait, CTX: fmt::Debug> fmt::Debug for Nitro<Err, CTX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &*self.inner;
        writeln!(f, "{:?}", inner)
    }
}

pub struct NitroInner<Err: StdErrorTrait, CTX> {
    context: CTX,
    error_trace: Frame,
    _type: PhantomData<Err>,
}

impl<Err: StdErrorTrait, CTX: fmt::Debug> fmt::Debug for NitroInner<Err, CTX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "context: {:?}", self.context)?;
        writeln!(f, "error backtrace: {:?}", self.error_trace)?;
        Ok(())
    }
}

pub struct Frame {
    error: Box<dyn StdErrorTrait>,
    file: &'static str,
    line: u32,
    column: u32,
    child: Option<Box<Frame>>,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_frame = self;
        for i in 0.. {
            write!(
                f,
                "\n {i:>2} | [{} {}:{}]: {}",
                last_frame.file, last_frame.line, last_frame.column, &*last_frame.error
            )?;

            match last_frame.child {
                Some(ref e) => {
                    last_frame = e;
                }
                None => break,
            }
        }

        Ok(())
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*self.error)
    }
}

impl Frame {
    #[track_caller]
    pub fn new<Error: StdErrorTrait + 'static>(err: Error) -> Self {
        let location = Location::caller();

        let file = location.file();
        let line = location.line();
        let column = location.column();

        Frame {
            error: Box::new(err),
            file,
            line,
            column,
            child: None,
        }
    }
}

impl<Err: StdErrorTrait + 'static, CTX> Nitro<Err, CTX> {
    #[track_caller]
    pub fn new(err: Err, initial_context: CTX) -> Self {
        let frame = Frame::new(err);

        let inner = Box::new(NitroInner {
            context: initial_context,
            error_trace: frame,
            _type: PhantomData,
        });
        Self { inner }
    }

    pub fn map_ctx<F, NewCTX>(self, f: F) -> Nitro<Err, NewCTX>
    where
        F: FnOnce(CTX) -> NewCTX,
    {
        let old = *self.inner;
        Nitro {
            inner: Box::new(NitroInner {
                context: f(old.context),
                error_trace: old.error_trace,
                _type: PhantomData,
            }),
        }
    }

    #[track_caller]
    pub fn raise<NewError: StdErrorTrait + 'static>(self, err: NewError) -> Nitro<NewError, CTX> {
        let old = *self.inner;
        let frame = {
            let mut frame = Frame::new(err);
            frame.child = Some(Box::new(old.error_trace));
            frame
        };

        Nitro {
            inner: Box::new(NitroInner {
                context: old.context,
                error_trace: frame,
                _type: PhantomData,
            }),
        }
    }

    pub fn as_error(&self) -> &Err {
        unsafe {
            self.inner
                .error_trace
                .error
                .downcast_ref()
                .unwrap_unchecked()
        }
    }
}

impl<Err: StdErrorTrait + 'static> Nitro<Err, ()> {
    #[track_caller]
    pub fn without_ctx(err: Err) -> Self {
        Self::new(err, ())
    }
}

impl<Err: StdErrorTrait + 'static> From<Err> for Nitro<Err, ()> {
    #[track_caller]
    fn from(err: Err) -> Self {
        Nitro::without_ctx(err)
    }
}
