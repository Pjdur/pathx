use std::path::Path;

/// Returns `true` if `child` is a subpath of `base`, lexically.
///
/// This does not access the filesystem and does not resolve symlinks.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use pathx::is_subpath;
///
/// let base = Path::new("/home/user/project");
/// let child = Path::new("/home/user/project/src/main.rs");
/// assert!(is_subpath(base, child));
/// ```
pub fn is_subpath(base: &Path, child: &Path) -> bool {
    let mut base_components = base.components();
    let mut child_components = child.components();

    while let (Some(b), Some(c)) = (base_components.next(), child_components.next()) {
        if b != c {
            return false;
        }
    }

    // If we exhausted all base components, it's a prefix of child
    base_components.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_subpath_true() {
        let base = Path::new("/a/b");
        let child = Path::new("/a/b/c/d.txt");
        assert!(is_subpath(base, child));
    }

    #[test]
    fn test_subpath_false() {
        let base = Path::new("/a/b");
        let child = Path::new("/a/x/c.txt");
        assert!(!is_subpath(base, child));
    }

    #[test]
    fn test_subpath_equal() {
        let base = Path::new("/a/b");
        let child = Path::new("/a/b");
        assert!(is_subpath(base, child));
    }

    #[test]
    fn test_subpath_relative() {
        let base = Path::new("foo/bar");
        let child = Path::new("foo/bar/baz");
        assert!(is_subpath(base, child));
    }

    #[test]
    fn test_subpath_not_prefix() {
        let base = Path::new("/foo/bar");
        let child = Path::new("/foo/barbaz");
        assert!(!is_subpath(base, child));
    }
}
