# Random key generator




## What

Generate random keys in handy way. 


## Why

To learn Rust.


## Requirement

Rust 1.39 or higher.


## Try it

```shell script
$ git clone https://github.com/TENX-S/rand_key
$ cd rand_key

# Default case: amount of letters: 10, symbols: 2, numbers: 3
$ cargo run --release --example kg_test

# Specify the parameter: amount of letters: 16, symbols: 2, numbers: 3
$ cargo run --release --example kg_test 16 2 3

# Try a larger number!
$ cargo run --release --example kg_test 200000 200 300

# Larger and set the unit value
$ cargo run --release --example kg_test 100000000 0 0 100
```


## ATTENTION!

**Do not** try a very large number, even it's allowed, like `i128::MAX`, unless you got extremely large RAM and great CPU, or you may got a **blue screen or any unpredictable behaviour** that will harm your computer hardware or unsaved files since you don't [set a right `UNIT` number](https://docs.rs/rand_pwd/1.1.3/rand_pwd/#the-unit-field).


## Usage

In `Cargo.toml`:
```toml
rand_pwd = "1"
```

You may want to use the latest feature(not stable and may requires nightly Rust):
```toml
rand_pwd = { git = "https://github.com/TENX-S/rand_key", branch = "master" }
```

Here's the simply demo of partital API:
```rust
use rand_key::{ RandKey, ToRandKey };

fn main() {

    let mut r_p = RandKey::new(10, 2, 3); // For now, it's empty. Use method `join` to generate the key
    r_p.join();                           // Now `r_p` has some content, be kept in its `content` field
    println!("{}", r_p);                  // Print it on the screen
    // One possible output: 7$pA7yMCw=2DPGN

    // Or you can build from an existing `&str`
    let mut r_p = RandKey::from("=tE)n5f`sidR>BV"); // 10 letters, 4 symbols, 1 number
    // You can re-generate a random key and with equivalent amount of letters, symbols and numbers. Like below:
    r_p.join();
    println!("{}", r_p);
    // One possible output: qS`Xlyhpmg~"V8[

    // You can also use the method `to_randkey` to convert a `String` or `&str` to `RandPwd`

    let mut r_p = "n4jpstv$dI,.z'K".to_randkey();

    // Panic! Has non-ASCII character(s)!
    // let mut r_p = RandPwd::from("🦀️🦀️🦀️");
    // let mut r_p = "🦀️🦀️🦀️".to_randkey();
}
```


## LICENSE
MIT
