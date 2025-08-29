use pathx::strip_root;
use std::path::PathBuf;

#[test]
fn strip_unix_root() {
    let path = PathBuf::from("/usr/local/bin");
    let stripped = strip_root(&path);
    assert_eq!(stripped, PathBuf::from("usr/local/bin"));
}
