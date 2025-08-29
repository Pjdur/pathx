use pathx::is_subpath;
use std::path::Path;

fn main() {
    // Check if a path is a subpath of another
    let base = Path::new("/home/user/project");
    let child = Path::new("/home/user/project/src/main.rs");
    println!("Is subpath: {}", is_subpath(base, child));
}
