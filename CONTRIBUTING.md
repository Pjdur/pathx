# Contributing to `pathx`

Thanks for your interest in contributing to `pathx`! This crate provides ergonomic, cross-platform utilities for working with paths in Rust, and we welcome improvements, bug fixes, and new ideas.

---

## 🧰 Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/pathx.git
   cd pathx
   ```
3. **Create a new branch** for your changes:
   ```bash
   git checkout -b feature/my-new-feature
   ```

---

## 🧪 Running Tests

Before submitting a pull request, make sure all tests pass:

```bash
cargo test
```

To check formatting and linting:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 📦 Platform Compatibility

`pathx` is designed to work on Windows, macOS, and Linux. If your contribution involves platform-specific logic, please:

- Use `#[cfg(windows)]` or `#[cfg(unix)]` where appropriate.
- Add tests for each platform if possible.

---

## 🧠 Code Style

- Follow Rust’s standard formatting (`rustfmt`).
- Prefer semantic naming and clear comments.
- Keep functions small and focused.
- Write tests for new functionality.

---

## 📝 Submitting a Pull Request

1. Push your branch to GitHub.
2. Open a pull request against `main`.
3. Include a clear description of your changes.
4. Reference any related issues (if applicable).

---

## 💡 Suggestions & Issues

If you have an idea or find a bug, feel free to [open an issue](https://github.com/Pjdur/pathx/issues). Please include:

- A clear title and description
- Steps to reproduce (if it's a bug)
- Your platform and Rust version

---

## 📄 License

By contributing, you agree that your code will be licensed under the [MIT License](LICENSE).

---

Thanks again for helping improve `pathx`! Your contributions make this crate better for everyone.
