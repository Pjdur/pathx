use pathx::is_subpath;
use std::path::Path;

#[test]
fn subpath_true() {
    let base = Path::new("/a/b");
    let child = Path::new("/a/b/c/d.txt");
    assert!(is_subpath(base, child));
}

#[test]
fn subpath_false() {
    let base = Path::new("/a/b");
    let child = Path::new("/a/x/c.txt");
    assert!(!is_subpath(base, child));
}
