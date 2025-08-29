use std::path::{Path, PathBuf};

/// Computes the relative path from `base` to `target`, lexically.
///
/// # Example
/// ```
/// use std::path::{Path, PathBuf};
/// use pathx::relative_to;
///
/// let base = Path::new("/a/b");
/// let target = Path::new("/a/b/c/d.txt");
/// let rel = relative_to(base, target);
/// assert_eq!(rel, PathBuf::from("c/d.txt"));
/// ```
pub fn relative_to(base: &Path, target: &Path) -> PathBuf {
    let base_components: Vec<_> = base.components().collect();
    let target_components: Vec<_> = target.components().collect();

    let mut i = 0;
    while i < base_components.len() && i < target_components.len() {
        if base_components[i] != target_components[i] {
            break;
        }
        i += 1;
    }

    let mut result = PathBuf::new();
    for _ in i..base_components.len() {
        result.push("..");
    }
    for comp in &target_components[i..] {
        result.push(comp.as_os_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_relative_to_nested() {
        let base = Path::new("/a/b");
        let target = Path::new("/a/b/c/d.txt");
        let rel = relative_to(base, target);
        assert_eq!(rel, PathBuf::from("c/d.txt"));
    }

    #[test]
    fn test_relative_to_sibling() {
        let base = Path::new("/a/b/c");
        let target = Path::new("/a/b/d/e.txt");
        let rel = relative_to(base, target);

        let expected = PathBuf::from_iter(["..", "d", "e.txt"]);
        assert_eq!(rel, expected);
    }

    #[test]
    fn test_relative_to_same() {
        let base = Path::new("/a/b");
        let target = Path::new("/a/b");
        let rel = relative_to(base, target);
        assert_eq!(rel, PathBuf::from(""));
    }
}
