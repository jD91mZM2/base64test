# Base64test

I just wanted to stop feeling like Base64 is black magic, so I made a Base64 implementation.

## Usage?

Absolutely not. This crate is not specifically optimized for anything.
I literally just made this as a test.

Please just use [base64](https://crates.io/crates/base64) instead of this.

------------------------

Encode:

```Rust
let encoded = base64test::encode(&*b"Man"); // Wikipedia example
assert_eq!(encoded, "TWFu");
```

Decode:

```Rust
let decoded = base64test::decode("TWFu"); // Wikipedia example
assert_eq!(decoded, b"Man");
```
