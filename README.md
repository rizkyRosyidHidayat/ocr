# ocr

![GitHub stars](https://img.shields.io/github/stars/rizkyRosyidHidayat/ocr?style=for-the-badge&logo=github) ![GitHub forks](https://img.shields.io/github/forks/rizkyRosyidHidayat/ocr?style=for-the-badge&logo=github) ![GitHub issues](https://img.shields.io/github/issues/rizkyRosyidHidayat/ocr?style=for-the-badge&logo=github) ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## 📑 Table of Contents

- [Description](#description)
- [Tech Stack](#tech-stack)
- [Quick Start](#quick-start)
- [Key Dependencies](#key-dependencies)
- [Run Commands](#run-commands)
- [Screenshots](#screenshots)
- [Project Structure](#project-structure)
- [Development Setup](#development-setup)
- [Contributing](#contributing)

## 📝 Description

A high-performance Optical Character Recognition (OCR) engine built from the ground up in Rust. This project leverages Rust's memory safety and zero-cost abstractions to provide a fast and reliable solution for extracting text from images and scanned documents. Designed for developers who need a robust, efficient, and thread-safe OCR tool, it prioritizes speed and accuracy in modern document processing workflows.

## 🛠️ Tech Stack

- 🦀 Rust

## ⚡ Quick Start

```bash

# Clone the repository
git clone https://github.com/rizkyRosyidHidayat/ocr.git

# Build and run
cargo run
```

## 📦 Key Dependencies

```
axum: { version
tokio: { version
tower-http: { version
ocrs: 0.12.2
rten: 0.24.0
rten-tensor: 0.24.0
rten-imageproc: 0.24.0
image: { version
serde: { version
serde_json: 1.0
anyhow: 1.0
url: 2.5
ureq: 3.3
home: 0.5
dotenvy: 0.15
```

## 🚀 Run Commands

- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`

## 📸 Screenshots

> **Tip:** You can auto-generate a beautiful project mockup image using the **Screenshot** button above!

<p align="center">
  <img src="https://via.placeholder.com/800x400?text=Main+Application+View" alt="Main Application View" width="80%"/>
</p>

<p align="center">
  <img src="https://via.placeholder.com/800x400?text=Feature+Showcase" alt="Feature Showcase" width="80%"/>
</p>

## 📁 Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── models
│   ├── text-detection.rten
│   └── text-recognition.rten
└── src
    ├── error.rs
    ├── main.rs
    ├── models.rs
    ├── ocr.rs
    └── output.rs
```

## 🛠️ Development Setup

### Rust Setup
1. Install Rust (via rustup: https://rustup.rs/)
2. Install dependencies: `cargo build`
3. Run the project: `cargo run`

## 👥 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/rizkyRosyidHidayat/ocr.git`
3. **Create** a new branch: `git checkout -b feature/your-feature`
4. **Commit** your changes: `git commit -am 'Add some feature'`
5. **Push** to your branch: `git push origin feature/your-feature`
6. **Open** a pull request

Please ensure your code follows the project's style guidelines and includes tests where applicable.

---
*This README was generated with ❤️ by [ReadmeBuddy](https://readmebuddy.com)*
