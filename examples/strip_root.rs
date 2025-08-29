use std::path::{Path};
use pathx::strip_root;

fn main() {
    // Strip root
    let base = Path::new("/home/user/project");
    let stripped = strip_root(base);
    println!("Stripped path: {stripped:?}");
}