use pathx::strip_root;
use std::path::Path;

fn main() {
    // Strip root
    let base = Path::new("/home/user/project");
    let stripped = strip_root(base);
    println!("Stripped path: {stripped:?}");
}
