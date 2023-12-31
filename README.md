# Ruxios 🚀

Ruxios is a lightweight yet powerful HTTP request library for Rust. Inspired by libraries like Axios, Ruxios provides a straightforward but mighty interface for your Rust applications.

<div align="center">
    <a href="https://github.com/devzolo/ruxios" rel="nofollow noopener noreferrer">
        <img src="https://github.com/devzolo/ruxios/raw/HEAD/docs/assets/logo.webp?sanitize=true" alt="Ruxios logo" width="300">
    </a>
</div>

[![Build Status](https://img.shields.io/github/workflow/status/devzolo/ruxios/CI)](https://github.com/devzolo/ruxios/actions)
[![Crates.io Version](https://img.shields.io/crates/v/ruxios.svg)](https://crates.io/crates/ruxios)
[![Documentation](https://docs.rs/ruxios/badge.svg)](https://docs.rs/ruxios)
[![License](https://img.shields.io/crates/l/ruxios.svg)](https://github.com/devzolo/ruxios/blob/main/LICENSE)

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ruxios = { version = "0.1.2" }
tokio = { version = "1.33.0", features = ["full"] }
serde = { version = "1.0.189", features = ["derive"] }
serde_json = "1.0.107"
```

## 🚀 Quick Start

```rust
use ruxios::prelude::*;

let api = Ruxios::from(RuxiosConfig {
    base_url: String::from("https://api.mysite.com"),
    ..Default::default()
});

let res = api.get::<Value, Value>("/my-route").await;

match res {
    Ok(res) => println!("{:?}", res.data),
    Err(err) => println!("{:?}", err),
}
```

## 💡 Features

- Clear and concise HTTP methods.
- Support for custom types for requests and responses.
- Simplified error handling.
- `fetch!` macro for quick GET requests.

## 📖 Documentation

For a deeper dive, check out our [full documentation](https://docs.rs/ruxios).

## 🌱 Contributing

1. Fork the repository.
2. Create a new branch for your changes: `git checkout -b my-new-feature`.
3. Commit your changes: `git commit -am 'Add some feature'`.
4. Push to the branch: `git push origin my-new-feature`.
5. Submit a pull request.

## 📝 License

This library is licensed under the MIT License. See the [LICENSE](https://github.com/devzolo/ruxios/blob/main/LICENSE) file for details.
