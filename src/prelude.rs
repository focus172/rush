pub use resu::{Context, Report, Result, ResultExt};
pub use std::{fmt, io};

pub use crate::lexer::Token;
pub use crate::shell::Shell;

// #[cfg(feature = "log")]

pub(crate) use self::logger::*;

#[cfg(feature = "log")]
mod logger {
    #[macro_export]
    macro_rules! log {
        ($($arg:tt)*) => {{
            paris::log!("<yellow><bold>[{module:<11}]<blue>::<green>{line:<4}</> <dimmed>#</> {}",
                format_args!($($arg)*),
                module=module_path!(),
                line=line!(),
            );
        }}
    }

    pub(crate) use crate::log;
    pub(crate) use paris::error;
    pub(crate) use paris::info;
    pub(crate) use paris::success;
    pub(crate) use paris::warn;
}

#[cfg(not(feature = "log"))]
mod logger {
    #[macro_export]
    macro_rules! log {
        ($($arg:expr),+) => {{
            $(let _ = $arg;)*
        }}
    }

    pub(crate) use self::log as error;
    pub(crate) use self::log as info;
    pub(crate) use self::log as success;
    pub(crate) use self::log as warn;
    pub(crate) use crate::log;
}
