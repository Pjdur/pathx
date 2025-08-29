use std::path::{Path, PathBuf};

use crate::utils::normalize::{Normalize, NormalizeError};

/// Joins two paths and normalizes the result lexically.
///
/// This is equivalent to `base.join(segment).normalize()`, but wrapped for convenience.
///
/// # Errors
///
/// Returns [`NormalizeError`] if the resulting path attempts to traverse above its root.
pub fn join(base: &Path, segment: &Path) -> Result<PathBuf, NormalizeError> {
    let combined = base.join(segment);
    combined.normalize()
}

/// A convenience version of [`join`] that returns the original joined path if normalization fails.
pub fn join_lossy(base: &Path, segment: &Path) -> PathBuf {
    join(base, segment).unwrap_or_else(|_| base.join(segment))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_join_normal_case() {
        let base = Path::new("/foo/bar");
        let segment = Path::new("../baz");
        let result = join(base, segment).unwrap();
        assert_eq!(result, PathBuf::from("/foo/baz"));
    }

    #[test]
    fn test_join_lossy_fallback() {
        let base = Path::new("../foo");
        let segment = Path::new("bar");
        let result = join_lossy(base, segment);
        assert_eq!(result, PathBuf::from("../foo/bar"));
    }

    #[test]
    fn test_join_rooted_segment() {
        let base = Path::new("/foo");
        let segment = Path::new("/bar/baz");
        let result = join(base, segment).unwrap();
        assert_eq!(result, PathBuf::from("/bar/baz"));
    }
}
