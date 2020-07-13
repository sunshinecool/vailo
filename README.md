# vailo
Cloud native OS written in Rust

## Environment setup
Make sure you have cargo, rustup in PATH.
(Add source $HOME/.cargo/env to shell config)
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

## Running 
Make sure you installed qemu
`sudo apt-get install qemu`

Use xrun to run image in qemu
`cargo xrun`
