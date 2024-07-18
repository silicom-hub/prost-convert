# Prost convert

## FAQ

### Why not use the standard `From` and `TryFrom` trait?

We redefine our own conversion trait which might not seem logical.

The main reason is to support removing unwanted optional field

In rust we have

```rust
impl<T> From<T> for Option<T> { /**/}
```

which allows

```rust
let o: Option<u8> = Option::from(67);
assert_eq!(Some(67), o);
```

But if we have

```rust
struct U;

impl From<U> for T { /**/}
```

we don't have

```rust
impl<T, U> From<U> for Option<T>
where
    T: From<U>,
{
    fn from(value: U) -> Self {
        Some(value.into())
    }
}
```

We can't add this impl due to the orphan rule, so we add our own conversion trait.
It’s mostly transparent to the user because most of the time it will be impl through a derive 
macro.


## TODO

- impl for NonZero types from the std.
- Explore the possibility to use an associated type for the error.
- impl for `Bytes` https://docs.rs/prost/latest/prost/trait.Message.html#foreign-impls
- Prost support to switch from `HashMap` to `BTreeMap` so we should support `BTreeMap` also.
- Should we make a blanket impl for all the type in the std that impl From/TryFrom (ex u16 and u32). Useful when we have a native type (u16) that can’t be express in the proto. If we don't control the proto and and they define a uint64 and we want a u16 we could provide conversion too.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
