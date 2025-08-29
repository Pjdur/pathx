/// Returns the platform-specific path separator as a string slice.
pub fn separator() -> &'static str {
    if cfg!(windows) { "\\" } else { "/" }
}
