use std::path::Path;
use pathx::relative_to;

fn main() {
    let base = Path::new("/home/user/project");
    let target = Path::new("/home/user/project/src/main.rs");
    let relative = relative_to(target, base);
    println!("Relative path: {}", relative.display());
} 