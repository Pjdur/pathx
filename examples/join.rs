use pathx::{join, join_lossy};
use std::path::Path;

fn main() {
    // Joining paths
    let base = Path::new("/foo/bar");
    let segment = Path::new("../baz");
    let result = join(base, segment).unwrap();
    println!("{result:?}");

    // Joining paths (lossy)
    let lossy_result = join_lossy(base, segment);
    println!("{lossy_result:?}");
}
