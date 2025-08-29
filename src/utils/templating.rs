use std::collections::HashMap;
use std::path::PathBuf;

/// Renders a path template using `{key}` placeholders.
///
/// # Example
/// ```
/// use pathx::render_template;
/// use std::collections::HashMap;
/// use std::path::PathBuf;
///
/// let mut vars = HashMap::new();
/// vars.insert("module", "utils");
/// vars.insert("file", "normalize");
///
/// let path = render_template("src/{module}/{file}.rs", &vars);
/// assert_eq!(path, PathBuf::from("src/utils/normalize.rs"));
/// ```
pub fn render_template(template: &str, vars: &HashMap<&str, &str>) -> PathBuf {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut key = String::new();
            while let Some(&next) = chars.peek() {
                if next == '}' {
                    chars.next(); // consume '}'
                    break;
                }
                key.push(next);
                chars.next();
            }
            if let Some(val) = vars.get(key.as_str()) {
                result.push_str(val);
            } else {
                result.push_str(&format!("{{{}}}", key)); // leave placeholder untouched
            }
        } else {
            result.push(c);
        }
    }

    PathBuf::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_template_basic() {
        let mut vars = HashMap::new();
        vars.insert("module", "core");
        vars.insert("file", "lib");

        let path = render_template("src/{module}/{file}.rs", &vars);
        assert_eq!(path, PathBuf::from("src/core/lib.rs"));
    }

    #[test]
    fn test_template_missing_key() {
        let vars = HashMap::new();
        let path = render_template("src/{missing}/file.rs", &vars);
        assert_eq!(path, PathBuf::from("src/{missing}/file.rs"));
    }

    #[test]
    fn test_template_no_placeholders() {
        let vars = HashMap::new();
        let path = render_template("src/main.rs", &vars);
        assert_eq!(path, PathBuf::from("src/main.rs"));
    }
}
