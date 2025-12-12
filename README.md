
# Rust Learning

This repository contains exercises and practical examples following the book ["The Rust Programming Language"](https://doc.rust-lang.org/book).

## Getting Started

### 1. Install Rust

If you don't have Rust installed, run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then restart your terminal and check the installation:
```bash
rustc --version
cargo --version
```

### 2. Clone this repository

```bash
git clone https://github.com/santimartinezzgb/curso-c-principiantes.git
cd rust-learning
```

### 3. Install Git Large File Storage (LFS)

Some Rust projects may use large files (e.g., binaries, assets). Git LFS helps manage these efficiently, preventing repository bloat and speeding up cloning.

To install Git LFS:
```bash
sudo apt-get install git-lfs
git lfs install
```
To track large files in this project, for example:
```bash
git lfs track "*.bin"
git add .gitattributes
git commit -m "Track binary files with Git LFS"
```

### 4. Build and Run Examples

Navigate to any project folder and use Cargo:
```bash
cd 2-hello_cargo
cargo run
```

## Why use Git LFS?

- Keeps repository size manageable.
- Makes cloning and pulling faster.
- Prevents large files from being stored directly in Git history.


### Author
Santi Martínez