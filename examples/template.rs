use pathx::{render_template, template};
use std::collections::HashMap;

fn main() {
    // Template usage example
    let mut vars = HashMap::new();
    vars.insert("module", "utils");
    vars.insert("file", "normalize");
    let path = render_template("src/{module}/{file}.rs", &vars);
    println!("Generated path: {}", path.display());

    // Template using template!() macro
    let path = template!("src/{module}/{file}.rs", {
        module: "utils",
        file: "normalize"
    });
    println!("Generated path: {}", path.display());
}