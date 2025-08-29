use pathx::{join, join_lossy};
use std::path::Path;

#[test]
fn join_and_normalize() {
    let base = Path::new("/foo/bar");
    let segment = Path::new("../baz");
    let result = join(base, segment).unwrap();
    assert_eq!(result, Path::new("/foo/baz"));
}

#[test]
fn join_lossy_fallback() {
    let base = Path::new("../foo");
    let segment = Path::new("bar");
    let result = join_lossy(base, segment);
    assert_eq!(result, Path::new("../foo/bar"));
}
