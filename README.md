# Retris
 - Tetris game written in rust with WASM support
 - Uses [macroquad](https://github.com/not-fl3/macroquad/) for input, graphics and sound.

## Install 
 - Install [Rust](https://www.rust-lang.org/tools/install) development tools.
 - Clone the git repository and run `cargo run`
 
## Webassembly
 - For WASM install target `rustup target install wasm32-unknown-unknown`
 - Install the WASM server runner `cargo install wasm-server-runner`
 - To launch code run `cargo run --target wasm32-unknown-unknown`
 - Build WAS binary using `cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir ./out/ --target web ./target/ `

## License
 - Project distributed under MIT license available on the project repository.