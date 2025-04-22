# serde-inline-default [![ci](https://github.com/ByteDream/serde-inline-default/actions/workflows/ci.yml/badge.svg)](https://github.com/ByteDream/serde-inline-default/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/serde-inline-default)](https://crates.io/crates/serde-inline-default) [![crates.io downloads](https://img.shields.io/crates/d/serde-inline-default)](https://crates.io/crates/serde-inline-default) [![docs](https://img.shields.io/docsrs/serde-inline-default)](https://docs.rs/serde-inline-default/latest/serde_inline_default/)

Tiny crate to set default values for serde fields via inline attribute declaration.

## Overview

This crate is an approach to do what [serde-rs/serde#368](https://github.com/serde-rs/serde/issues/368) purposes.
If you want to set default values in plain [`serde`](https://serde.rs/), you have to create a function and link to it with `#[serde(default = "...")`.
This may be good if you need to do calculations to get the default value, but often you just want a simple integer or string to be the default value and have to create a whole function to return a hard-coded value.
```rust
#[derive(Deserialize)]
struct Test {
    #[serde(default = "value_default")]
    value: u32
}

fn value_default() -> u32 { 42 }
```

That can get quiet messy if you have many fields with many (different) default values.
This crate tries to solve this issue by providing the `#[serde_inline_default]` proc macro.
With this macro set at the struct level, you can set default values via `#[serde_inline_default(...)]` for your serde fields inline, without creating an extra function.

```rust
#[serde_inline_default]
#[derive(Deserialize)]
struct Test {
    #[serde_inline_default(42)]
    value: u32
}
```

> [!IMPORTANT]
> **`#[serde_inline_default]` must be set before `#[derive(Deserialize)]`/`#[derive(Serialize)]`, otherwise it's not working correctly!**

Internally, `#[serde_inline_default(...)]` gets expanded to a function which returns the set value and the attribute is replaced with `#[serde(default = "<function name>")]`.
So this macro is just some syntax sugar for you, but can get quiet handy if you want to keep your code clean or write declarative macros / `macro_rules!`.

## Alternatives

This crate isn't perfect. Thus, you might be more satisfied with alternatives `serde` provides.

With `#[serde(default)]` + `impl Default` on a struct, `serde` uses the default implementation of the struct to get default values for each field ([docs](https://serde.rs/container-attrs.html#default)):
```rust
#[derive(Deserialize)]
#[serde(default)]
struct Test {
    value: u32
}

impl Default for Test {
    fn default() -> Self {
        Self {
            value: 42
        }
    }
}
```

If you still need/want `serde-inline-default` features, you also can combine them with `#[serde(default))` and `impl Default`:
```rust
#[serde_inline_default]
#[derive(Deserialize)]
#[serde(default)]
struct Test {
    value: u32,
    #[serde_inline_default(0)]
    other_value: u32,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            value: 42,
            other_value: 42
        }
    }
}
```

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
