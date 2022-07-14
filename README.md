# chess-engine

# Description
This is a simple project that includes:
  - A chess library with validating and generating moves
  - A GUI (built with sdl2) to play chess using that library

I try to have as little dependencies as possible while also caring about performance (as much as my knowledge allows me to, at least).

This project's main goal is to help me get better at Rust. But that doesn't mean that contributing isn't allowed - **any contributions are welcomed** as they help me learn faster!

# Building
This project uses stable Rust (edition 2021), so to build it, just [install rustup](https://www.rust-lang.org/tools/install) and then run:
```
cd chess-engine
rustup default stable
rustup update
cargo build --release
```
