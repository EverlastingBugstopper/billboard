use std::str::from_utf8;
use strip_ansi_escapes::strip;
use thiserror::Error;
use unicode_width::UnicodeWidthStr;

/// Possible errors that can occur while calculating the visual width of a String.
#[derive(Error, Debug)]
pub enum VisualWidthError {
    #[error("could not strip ANSI codes from string")]
    AnsiStrip {
        #[from]
        source: std::io::Error,
    },
    #[error("could not construct string after stipping ANSI codes")]
    Utf8 {
        #[from]
        source: std::str::Utf8Error,
    },
}

pub(crate) fn visual_width(input: &str) -> Result<usize, VisualWidthError> {
    let no_ansi = strip(input)?;
    let no_ansi = from_utf8(&no_ansi)?;
    Ok(UnicodeWidthStr::width(no_ansi))
}
