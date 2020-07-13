# vailo
Cloud native OS written in Rust

## Environment setup
Install `cargo` (package manager for Rust (like npm)) and 
`rustup` (Version management for Rust (like nvm))
```
$ curl https://sh.rustup.rs -sSf | sh
```
Make sure you have cargo, rustup in PATH.
(Add `source $HOME/.cargo/env` to shell config (`~/.bash_profiles` or `~/.zshrc`)
```
cargo install cargo-xbuild
rustup component add rust-src
```
```
cargo install bootimage
rustup component add llvm-tools-preview
```
## Building project
`cargo xbuild`
Bootable image is stored in `target/x86_64-vailo/debug/`

## Running 
Install qemu (lightweight vm). see https://www.qemu.org/download/
`sudo apt-get install qemu` for ubuntu
`brew install qemu` for mac os

Run the bootable kernel
`cargo xrun`

