use pathx::normalize;
use std::path::Path;

fn main() {
    // Normalizing a path
    let path = Path::new("foo/./bar/../baz");
    let normalized = normalize(path).unwrap();
    println!("Normalized path: {normalized:?}");
}
