use pathx::{render_template, template};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn render_template_function() {
    let mut vars = HashMap::new();
    vars.insert("module", "utils");
    vars.insert("file", "normalize");

    let path = render_template("src/{module}/{file}.rs", &vars);
    assert_eq!(path, PathBuf::from("src/utils/normalize.rs"));
}

#[test]
fn render_template_macro() {
    let path = template!("src/{module}/{file}.rs", {
        module: "utils",
        file: "normalize"
    });

    assert_eq!(path, PathBuf::from("src/utils/normalize.rs"));
}
