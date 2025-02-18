# Cartest

This repo is a demonstration of how unit tests work in Rust

# How to Run

To Run the unit tests, you must have `cargo` and `rust-toolchain` installed. You can install it [here](https://www.rust-lang.org/tools/install)

If you are on windows make sure to install the GNU Toolchain instead of MSVC Toolchain
```bash
rustup toolchain install stable-gnu
rustup set default-host x86_64-pc-windows-gnu
```

To execute the unit tests, run the following command(s)

```bash
cargo test
```
