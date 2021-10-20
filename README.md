# Fateful
A tool to fatefully exit the process without panics

## Install
Add to your `cargo.toml` file
```toml
[dependencies]
fateful = "0.1.1"
```

## Usage
```rust
use std::env;
use rand;
use fateful::{fatal, err_prefix};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        fatal!(err_prefix!(), "missing random items to choose of");
    }

    let random_items: &Vec<String> = &args[1..args.len()].to_vec();

    let index = (rand::random::<f32>() * random_items.len() as f32).floor() as usize;
    println!("U need to study: {} 🎉", random_items[index]);
}
```
If u don't provide at least 2 arguments after `cargo run` the output will be:
```
Error: missing random items to choose of
```