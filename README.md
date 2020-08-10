# mdblocks

## Install

### with cargo install
To use cargo install, you should have your cargo bin directory (typically `$HOME/.cargo/bin`) in your path. Then from inside the crate root (where `Cargo.toml` is located) run 

```
cargo install --path .
```

### manually
From the crate root run

```
cargo build --release
```

then copy `./target/release/mdblocks` to whereever you want


<small>
note: you can also just run the program from the crate root using `cargo run`
</small>

## Usage
Extract all the text between code blocks in markdown and outputs it all to to stdout

If command line arguments are provided, mdblocks will read each as a file, and output any code blocks found, in the order of the arguments

```
mdblocks file1 [file2 [...]]
```

If no arguments are provided, mdblocks will read from stdin

```
cat file.txt | mdblocks
```
