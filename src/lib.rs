use std::fmt::{self, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct Trerr {
    err: String,
    cause: Option<Box<Trerr>>,
}

impl Display for Trerr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "error: {}", self.err)?;
        let mut indent = String::new();
        let mut cause = &self.cause;
        while let Some(e) = cause {
            indent += " ";
            writeln!(f, "{}caused by: {}", indent, e.err)?;
            cause = &e.cause;
        }
        Ok(())
    }
}

impl Error for Trerr {
    fn description(&self) -> &str { &self.err }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| e as &Error)
    }
}

#[macro_export]
macro_rules! trerr {
    ($cause:expr) => {
        trerr($cause, format!("[{}:{}]", file!(), line!()))
    };
    ($cause:expr, $fmt:expr) => {
        trerr($cause, format!(concat!("[{}:{}] ", $fmt), file!(), line!()))
    };
    ($cause:expr, $fmt:expr, $($arg:tt)*) => {
        trerr($cause, format!(concat!("[{}:{}] ", $fmt), file!(), line!(), $($arg)*))
    };
}

pub fn trerr(cause: Option<Trerr>, msg: String) -> Trerr {
    Trerr {
        err: msg,
        cause: cause.map(|e| Box::new(e)),
    }
}
