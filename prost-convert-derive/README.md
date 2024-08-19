# Prost convert derive

This crate provides a derive macro for `TryFromProto` and `FromNative` traits.

## What is required to use it.

- Proto struct/enum and native the native one should have the same name.
- Proto struct/enum and native the native one should have the same fields name.

## Usecase : remove unwanted option

Prost wrap user defined messages into optional, as stated by the proto3 specs.

One idea of the macro is that if the proto struct has a field which is optional and the native ones no, provide a conversion function that will try to do the conversion.

Sadly, we can't know in the derive macro if the proto struct field is optional.
We only got the struct path, [we can't simply go and parse it].


So there if we use `From` and `TryFrom`, there is no way to customize the behavior if the proto struct field is optional, because we don't know if it does.

> One solution will be to provide `#[required]` attribute on the field on the native struct but that's verbose and add some boilerplate.


Besides `TryFrom<Option<T>> for T` is not implemented and not implementable because of the orphan rule.

For instance, this won't work :

```rust
let option = Some(String::from("foo"));
let string: String = o.try_into()?;
```

So we make our own trait to be able to define what to do when converting from an `Opion<T>` into a `T`.

## Usecase : remove unwanted wrapper

 What we really need is to convert to the wrapper to send it over the network.
And be created from the wrapper after receiving it through the network.
So we must specify the wrapper path. But from the wrapper path we can't deduce
the wrapped path and vice versa.
If the native is an enum can we assume the src given is a struct which has only one field
which is an enum? -> No
Currently we need to impl ProstConvert for each nested struct.
So we can have enum which are not inside a wrapper struct.
If the wrapper makes just a recursive call, maybe it’s possible not to provide the wrapped?

Syntax for enum:

```rust
#[derive(ProstConvert)]
#[prost_convert(src = "proto::Foo", wrapper = "proto::FooWrapper")]
enum Foo {
    Foo1,
    Foo2,
}
```

This will generate 2 implementations of [`TryFromProto`] :
The "src" attribute will do the same as it does for the struct, it will generate:

```rust
impl TryFromProto<proto::Foo> for Foo
impl FromNative<Foo> for proto::Foo
```

But the "wrapper" attribute will also generate 2 more convertion function, with the wrapper struct:

```rust
impl TryFromProto<proto::FooWrapper> for Foo
impl FromNative<Foo> for proto::FooWrapper
```

If you use the wrapper keyword, no need to derive `ProstConvert` for the wrapper itself. In other words, we don't need

```rust
impl TryFromProto<proto::FooWrapper> for FooWrapper
impl FromNative<FooWrapper> for proto::FooWrapper
```

For now:
- wrapper can only be used on enum (easy to make it for struct but is there a use case?)
- wrapper assumes that the wrapper is a struct of one filed containing one enum.


## Potential improvement

- Currently this macro impl both `TryFromProto` and `FromNative`, but maybe sometimes we only want one of those.
- Just use darling for parsing macro argument (check if it integrates well with `syn::Error`).
- Compile error when a field doesn't impl prost derive should be on the field and not on the struct (like when missing Debug, Clone, etc..)
- compile error when `[prost_convert(src = "..")]` appear at the top of struct comment => shouldn't be the case.
- Does we use all the variants of the error? 
- Confirm that `Style::Tuple` and `Style::Tuple` can't be created from protobuf.
- Provide custom impl if wanted? For instance, to create  `std::net::IpAddress` from `std::string`.
- Handle new type `struct Id(Uuid)`.
- Handle tuple `struct Foo(i32, i32)`
- Add UT for this `pub struct Message {}`
- Handle "struct" enum variant or mark it in the doc. Beside if we don't support it, we can add a compile error on the macro.


[we can't simply go and parse it]: https://github.com/rust-lang/rust/issues/55904

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
