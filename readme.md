# slugi

[![Crates.io](https://img.shields.io/crates/v/slugi.svg)](https://crates.io/crates/slugi)
[![Docs.rs](https://docs.rs/slugi/badge.svg)](https://docs.rs/slugi)
[![License](https://img.shields.io/crates/l/slugi.svg)](https://github.com/Omchaudhary2004/slugify)

A simple, lightweight Rust library for turning text into **URL-safe slugs**.  
Inspired by tools like Python’s `slugify`, but designed for idiomatic Rust usage.
GITHUBLINK
```
https://github.com/Omchaudhary2004/slugify
```
---

## ✨ Features
- Convert accented characters (`Café → cafe`)  
- Lowercase all text  
- Replace spaces with `_`  
- Keep only ASCII alphanumeric characters and underscores  
- Tiny, dependency-free implementation  

---

## 📦 Installation

Add `slugi` to your `Cargo.toml`:

```toml
[dependencies]
slugi = "0.1"

Or install directly with Cargo:
```bash
cargo add slugi
```

🚀 Usage
```rust
use slugi::slugify;

fn main() {
    let text = "Café au lait! 2025";
    let slug = slugify(text);
    println!("Original: {}", text);
    println!("Slugified: {}", slug);
    // Output: cafe_au_lait_2025
}
```
📖 Documentation

Full API docs are available at docs.rs/slugi
.

✅ Roadmap

Support for more diacritics

Configurable replacement character (- instead of _)

Optional Unicode normalization

🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to check the issues page
.

Fork the repo

Create a new branch (git checkout -b feature/my-feature)

Commit changes (git commit -m 'Add my feature')

Push to the branch (git push origin feature/my-feature)

Open a Pull Request