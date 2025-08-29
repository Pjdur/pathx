use pathx::relative_to;
use std::path::{Path, PathBuf};

#[test]
fn relative_path_nested() {
    let base = Path::new("/a/b");
    let target = Path::new("/a/b/c/d.txt");
    let rel = relative_to(base, target);
    assert_eq!(rel, PathBuf::from_iter(["c", "d.txt"]));
}
