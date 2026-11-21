# Building the CLI

## Prerequisites

- Rust stable toolchain
- Native HID build dependencies:

```bash
# Debian/Ubuntu
sudo apt install build-essential pkg-config libudev-dev libhidapi-dev

# Fedora
sudo dnf install gcc pkgconf-pkg-config systemd-devel hidapi-devel

# Arch
sudo pacman -S base-devel pkgconf systemd hidapi
```

## Build

From the repository root:

```bash
cargo build --manifest-path src-tauri/Cargo.toml --release
```

The binary is created at `src-tauri/target/release/ols-cli`. Run it with:

```bash
src-tauri/target/release/ols-cli --brightness 10 --speed 1 --effect fire -power_on
```
