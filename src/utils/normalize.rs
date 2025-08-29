use std::path::{Component, Path, PathBuf};

/// Error type returned when path normalization fails.
#[derive(Debug, Clone)]
pub struct NormalizeError {
    pub message: &'static str,
}

impl std::fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path normalization failed: {}", self.message)
    }
}

impl std::error::Error for NormalizeError {}

/// Extension trait to add `normalize` to [`Path`].
pub trait Normalize {
    /// Normalize a path lexically, resolving `.` and `..` without accessing the filesystem.
    ///
    /// # Errors
    ///
    /// Returns [`NormalizeError`] if the path attempts to traverse above its root.
    fn normalize(&self) -> Result<PathBuf, NormalizeError>;
}

impl Normalize for Path {
    fn normalize(&self) -> Result<PathBuf, NormalizeError> {
        let mut lexical = PathBuf::new();
        let mut iter = self.components().peekable();

        let root_len = match iter.peek() {
            Some(Component::ParentDir) => {
                return Err(NormalizeError {
                    message: "cannot start with ParentDir",
                });
            }
            Some(Component::Prefix(prefix)) => {
                lexical.push(prefix.as_os_str());
                iter.next();
                if let Some(Component::RootDir) = iter.peek() {
                    lexical.push(Component::RootDir);
                    iter.next();
                }
                lexical.as_os_str().len()
            }
            Some(Component::RootDir) | Some(Component::CurDir) => {
                lexical.push(iter.next().unwrap());
                lexical.as_os_str().len()
            }
            None => return Ok(PathBuf::new()),
            Some(Component::Normal(_)) => 0,
        };

        for component in iter {
            match component {
                Component::RootDir => unreachable!(),
                Component::Prefix(_) => {
                    return Err(NormalizeError {
                        message: "unexpected prefix component",
                    });
                }
                Component::CurDir => continue,
                Component::ParentDir => {
                    if lexical.as_os_str().len() == root_len {
                        return Err(NormalizeError {
                            message: "attempted to traverse above root",
                        });
                    } else {
                        lexical.pop();
                    }
                }
                Component::Normal(path) => lexical.push(path),
            }
        }

        Ok(lexical)
    }
}

/// Normalize a path lexically, resolving `.` and `..` without accessing the filesystem.
///
/// # Errors
///
/// Returns [`NormalizeError`] if the path attempts to traverse above its root.
pub fn normalize(path: &Path) -> Result<PathBuf, NormalizeError> {
    Normalize::normalize(path)
}

/// A convenience function that returns the normalized path or the original path if normalization fails.
pub fn normalize_lossy(path: &Path) -> PathBuf {
    path.normalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_basic() {
        let path = Path::new("foo/./bar/../baz");
        let normalized = path.normalize().unwrap();
        assert_eq!(normalized, PathBuf::from("foo/baz"));
    }

    #[test]
    fn test_normalize_rooted() {
        let path = Path::new("/foo/../bar");
        let normalized = path.normalize().unwrap();
        assert_eq!(normalized, PathBuf::from("/bar"));
    }

    #[test]
    fn test_normalize_error() {
        let path = Path::new("../foo");
        let err = path.normalize().unwrap_err();
        assert_eq!(err.message, "cannot start with ParentDir");
    }

    #[test]
    fn test_normalize_lossy() {
        let path = Path::new("../foo");
        let normalized = normalize_lossy(path);
        assert_eq!(normalized, PathBuf::from("../foo"));
    }
}
