// pathx macros

/// template!() macro
#[macro_export]
macro_rules! template {
    ($template:expr, { $($key:ident : $val:expr),* $(,)? }) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert(stringify!($key), $val);
        )*
        $crate::utils::templating::render_template($template, &map)
    }};
}

// Test
#[cfg(test)]
mod tests {
    #[test]
    fn test_template_macro() {
        let path = template!("src/{dir}/{name}.rs", {
            dir: "core",
            name: "lib"
        });

        assert_eq!(path.to_str().unwrap(), "src/core/lib.rs");
    }
}
