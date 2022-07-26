use crate::core::EString;

/// The error type for operations interacting with ``EString``’s fragments.
#[derive(Debug, PartialEq, Eq)]
pub struct Error(pub EString, pub Reason);

/// The reason for the failure to parse.
#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    /// Cannot split fragment
    Split,
    /// Cannot parse fragment
    Parse,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Failed to parse "{:?}" with reason {:?}"#,
            self.0, self.1
        )
    }
}

impl std::error::Error for Error {}

impl std::ops::Deref for Error {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
