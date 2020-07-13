# vailo
Cloud native OS written in Rust

## Environment setup
#### Install Rust runtime
Install `cargo` (package manager for Rust (like npm)) and 
`rustup` (Version management for Rust (like nvm))
```
$ curl https://sh.rustup.rs -sSf | sh
```
Make sure you have cargo, rustup in PATH.
(Add `source $HOME/.cargo/env` to shell config (`~/.bash_profiles` or `~/.zshrc`)

#### Install project dependencies
##### xbuild for building without stdlib
```
cargo install cargo-xbuild
rustup component add rust-src
```
##### Bootimage for creating bootloader
```
cargo install bootimage
rustup component add llvm-tools-preview
```
##### Install qemu (lightweight vm) 
see https://www.qemu.org/download/
`sudo apt-get install qemu` for ubuntu
`brew install qemu` for mac os

## Building project
`cargo xbuild`
Bootable image is stored in `target/x86_64-vailo/debug/`

## Running 
Run the bootable kernel
`cargo xrun`

# Resources for learning
## Rust
https://www.rust-lang.org/learn has 3 paths for learning Rust. I tried the book approach, liked it. Will try others as well. 

## Develop OS in Rust 
https://os.phil-opp.com/ I am following this tutorial for getting hands on experience. Once we get to a good state, we can switch to use a light weight verions of https://www.redox-os.org/ 

## Why Rust 
* Read here - https://doc.redox-os.org/book/ch01-07-why-rust.html
* Also because of WebAssembly. My goal for this OS is close to Chrome OS. Using web frameworks makes the development of GUI fast. I plan to make v8 work and then leverage v8 server + webassembly + JS 
