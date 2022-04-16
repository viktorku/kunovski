## Outline

My personal (home)page built with WebAssembly and Rust. Hosted at [http://viktor.kunovski.com/](http://viktor.kunovski.com/).

It relies on `wasm-bindgen`, a [Rust](https://www.rust-lang.org/) library for writing efficient WebAssembly libraries with Rust and not having to worry (too much) about the low-level WASM-JS interop.

It was initially done to learn more about WASM and its capabilities and to up my skills in Rust [in mid-2018].

## How it works

It requires the book Moby Dick as a plain text file (fetched from [Project Gutenberg](https://www.gutenberg.org/ebooks/2701)) and a 'description' file containing a few short and obscure sentences about me.

The code uses Rust via WASM to access the book contents, extracts all words as a corpus, and asynchronously constructs my name by choosing the words whose first letter matches any of the letters in my name. The words -- picked randomly -- that don't match the appropriate first-letter structure are skipped but the incorrect letter is still briefly shown. This gives a somewhat cool Matrix-like shuffling effect.

Similarly, the 'description words' start shuffling and showing random words if you hover over them.

## Install

Install [Rust](https://rustup.rs/). The stable channel toolchain is sufficient.

Add the WASM target and install the wasm-bindgen CLI

```sh
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

Run the `serve` or `build` scripts to run a local instance or build prod assets, respectively.

## Caveats/TODO

The following things should be tackled at some point:

- The code uses an old and possibly outdated version of `wasm-bindgen`.
- It doesn't rely on `js-sys` and/or `web-sys` for simpler binding JS or browser env definitions (`setTimeout`, `performance` etc).
- Update to the latest Rust edition (2021 at time of writing)
- Bundler (WebPack) improvements
- Better project structure (wasm-pack?)
- Replace JS with TypeScript
