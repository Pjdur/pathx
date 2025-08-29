use std::path::{Component, Path, PathBuf};

/// Strips the root or prefix from a path, returning a relative version.
///
/// This is purely lexical and does not access the filesystem.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use pathx::strip_root;
///
/// let path = Path::new("/usr/local/bin");
/// let stripped = strip_root(path);
/// assert_eq!(stripped, PathBuf::from("usr/local/bin"));
/// ```
pub fn strip_root(path: &Path) -> PathBuf {
    let mut components = path.components();

    // Skip prefix (Windows) and root (Unix or Windows)
    while let Some(c) = components.next() {
        match c {
            Component::Prefix(_) | Component::RootDir => continue,
            _ => {
                // First non-root component found
                let mut result = PathBuf::new();
                result.push(c.as_os_str());
                for comp in components {
                    result.push(comp.as_os_str());
                }
                return result;
            }
        }
    }

    PathBuf::new()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_strip_root_unix() {
        let path = Path::new("/foo/bar");
        let stripped = strip_root(path);
        assert_eq!(stripped, PathBuf::from("foo/bar"));
    }

    #[test]
    fn test_strip_root_windows_prefix() {
        let path = Path::new("C:\\foo\\bar");
        let stripped = strip_root(path);
        assert_eq!(stripped, PathBuf::from("foo\\bar"));
    }

    #[test]
    fn test_strip_root_relative() {
        let path = Path::new("foo/bar");
        let stripped = strip_root(path);
        assert_eq!(stripped, PathBuf::from("foo/bar"));
    }

    #[test]
    fn test_strip_root_only_root() {
        let path = Path::new("/");
        let stripped = strip_root(path);
        assert_eq!(stripped, PathBuf::new());
    }
}
