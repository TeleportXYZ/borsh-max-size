# MaxSize

A Rust library that provides a `MaxSize` trait, which gives the maximum size the borsh serialization of a struct or enum could be in bytes. Implementations for simple types and a derive macro are included to make it easy to use with your custom types.

## Features

- `MaxSize` trait: Get the maximum borsh serialized size of a type in bytes.
- Implementations for simple types: Integers, floats, etc.
- Derive macro: Automatically implement `MaxSize` for your structs and enums.

## Usage
1. Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
borsh-max-size = "0.1"
```

`borsh-max-size` contains optional `arrayvec` and `solana-program` features that add support for those libraries.

2. Use the `MaxSize` trait in your code:
```rust
use borsh_max_size::MaxSize;

let max_size = i32::max_size(); // for simple types
assert_eq!(max_size, 4);
```

3. Derive `MaxSize` for your own types
```rust
use borsh_max_size::MaxSize;

#[derive(MaxSize)]
struct MyStruct {
    a: i32,
    b: u64,
    c: [u8; 4]
}

println!("Max size of MyStruct: {}", MyStruct::max_size());
```

## License

This project is licensed under the MIT License. See the LICENSE.md file for details.
