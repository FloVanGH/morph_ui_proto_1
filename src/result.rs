/// Max length of strings: 20 characters
// type MaxStringLength = heapless::consts::U20;
// /// String is now redefined as a fixed-size string
// type String = heapless::String::<MaxStringLength>;

/// Represents custom morph result.
pub type MorphResult<T> = Result<T, MorphError>;

/// Represents an custom morph error and is used by `MorphResult`.
#[derive(Clone, Debug)]
pub enum MorphError {
    /// Backend (platform specific) error
    Backend(&'static str),

    /// Not specified morph error.
    Other(&'static str)
}