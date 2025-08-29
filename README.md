<h1 align="center">pathx</h1>

<p align="center">
  <a href="https://crates.io/crates/pathx">
    <img src="https://img.shields.io/crates/v/pathx.svg?style=flat-square" alt="Crates.io">
  </a>
  <a href="https://docs.rs/pathx">
    <img src="https://img.shields.io/badge/docs.rs-pathx-blue?style=flat-square" alt="Docs.rs">
  </a>
  <a href="https://github.com/Pjdur/pathx/blob/main/LICENSE">
    <img src="https://img.shields.io/crates/l/pathx?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Pjdur/pathx/blob/main/.github/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/pjdur/pathx/ci.yml?style=flat-square" alt="CI">
  </a>
</p>

<p align="center"><b>A collection of ergonomic, cross-platform utilities for working with paths in Rust.</b></p>

From lexical normalization to templated path generation, `pathx` helps you compose, analyze, and manipulate paths with clarity and precision.

---

## ✨ Features

- `join()` — Join and normalize paths lexically
- `normalize()` — Resolve `.` and `..` without touching the filesystem
- `relative_to()` — Compute relative paths between two locations
- `template!()` — Rust-style macro for path templating
- `is_subpath()` — Check if one path is lexically nested under another
- `strip_root()` — Remove root or prefix from a path
- Platform-aware separator via `separator()`

All utilities are **pure**, **lexical**, and **cross-platform safe**.

---

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
pathx = "0.1.0"
```

---

## 🛠️ Usage Examples

### Join and Normalize

```rust
use pathx::join;
use std::path::Path;

let base = Path::new("/foo/bar");
let segment = Path::new("../baz");
let path = join(base, segment).unwrap();
// → "/foo/baz"
```

### Path Templating

```rust
use pathx::template;

let path = template!("src/{module}/{file}.rs", {
    module: "utils",
    file: "normalize"
});
// → "src/utils/normalize.rs"
```

### Relative Path

```rust
use pathx::relative_to;
use std::path::Path;

let base = Path::new("/a/b/c");
let target = Path::new("/a/b/d/e.txt");
let rel = relative_to(base, target);
// → "../d/e.txt"
```

---

## 📚 API Reference

| Function / Macro            | Description                                    |
| --------------------------- | ---------------------------------------------- |
| `join(base, segment)`       | Joins and normalizes two paths                 |
| `join_lossy(...)`           | Same as `join`, but falls back on error        |
| `normalize(path)`           | Lexically resolves `.` and `..`                |
| `normalize_lossy(...)`      | Returns original path if normalization fails   |
| `relative_to(base, target)` | Computes relative path from base to target     |
| `is_subpath(base, child)`   | Checks if `child` is nested under `base`       |
| `strip_root(path)`          | Removes root or prefix from a path             |
| `template!(...)`            | Macro for path templating using `{key}` syntax |
| `render_template(...)`      | Function version of `template!()`              |
| `separator()`               | Returns platform-specific path separator       |

---

## 📦 Platform Support

- ✅ Windows
- ✅ macOS
- ✅ Linux

All path operations are **lexical only** — no filesystem access or symlink resolution.

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!

1. Fork the repo
2. Create a feature branch
3. Write tests for your changes
4. Submit a pull request

Please follow Rust’s formatting and documentation conventions.

---

## 📄 License

Licensed under the [MIT License](LICENSE).  
You’re free to use, modify, and distribute this crate with attribution.

---

## 💬 Author

Created by [Pjdur](https://github.com/Pjdur).  
Feel free to reach out or open an issue if you have ideas or feedback!
