use pathx::Normalize;
use std::path::Path;

#[test]
fn normalize_basic() {
    let path = Path::new("foo/./bar/../baz");
    let normalized = path.normalize().unwrap();
    assert_eq!(normalized, Path::new("foo/baz"));
}

#[test]
fn normalize_error() {
    let path = Path::new("../foo");
    assert!(path.normalize().is_err());
}
