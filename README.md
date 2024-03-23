# learning-rust

## Install RustUp
update apt 
`sudo apt update`

install curl 
`sudo apt install curl`

install rust up 
`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

install c compiler
`sudo apt install build-essential`
`gcc --version`

verify rust
*close current terminal*
`rustc --version`

update rust (in the future)
`rustup update`

## Install VS Code
Use GUI installer

## Install NeoVim
`sudo apt update`
`sudo apt install neovim`

## Setup GitHub
`ssh-keygen -t ed25519 -C "user@email.com"`
`git clone git@github.com...`
`git config --global user.email "user@email.com"`
`git config --global user.name "username"`
`git config --global core.editor "nvim"`

## Rust!

### compile and run
`rustc main.rs` compiles the file main.rs into main
`./main` runs the binary

### macro vs function
`println!` is a macro call
`println` is a function call

### cargo
cargo is a build system and package manager
`cargo new project_name` initializes new project

`cargo build` builds project, puts binary in debug folder by default
`./target/debug/project_name` runs the binary

but wait, its easier with cargo
`cargo run` will build the project and run the generated binary without having to navigate to find it

`cargo check` will ensure the project compiles but will not produce an executable

ready for release? `cargo build --release`

### rust language features
there is a set of items in the standard library that is brought into each program by default, this is called the *prelude*
to include types not included in the prelude use `use`

### data types
8 bit integer - i8 u8
16 bit integer - i16 u16
32 bit integer - i32 u32
64 bit integer - i64 u64
128 bit integer - i128 u128
architechture bit integer - isize usize
32 bit floating point - f32
64 bit floating point - f64
boolean values - true false
characters - char