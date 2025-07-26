/// Error indicating that attempting to convert a raw address to a structured
/// `Address<T>` found non-canonical bits.
#[derive(Debug, PartialEq, Eq)]
pub struct NonCanonicalError;

impl core::error::Error for NonCanonicalError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl core::fmt::Display for NonCanonicalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("attempt to convert a raw address found non-canonical bits")
    }
}
