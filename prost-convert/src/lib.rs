//! Traits for conversions between native and proto types.
//!

// TODO: change it to `warn(..)` if we go open source. Indeed `deny(..)` could break user code if it uses a
// newer version of rust with new warnings)
#![deny(
    clippy::all,
    clippy::cargo,
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]
// FIXME: upgrade syn to 2.0
#![allow(clippy::multiple_crate_versions)]

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::num::TryFromIntError;
use std::path::PathBuf;
use std::{collections::HashMap, net::IpAddr};

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`IntoProto`].
///
/// One should always prefer implementing `FromNative` over [`IntoProto`]
/// because implementing `FromNative` automatically provides one with an implementation of [`IntoProto`]
/// thanks to the blanket implementation in this crate.
///
/// This can't failed because every prost type is just a subset of the native one.
///
/// # Generic Implementations
///
/// - `FromNative<T> for U` implies [`IntoProto`]`<U> for T`
///
/// You should probabily use the derive macro to impl `FromNative<P>`
pub trait FromNative<N>: Sized {
    /// Performs the conversion.
    fn from_native(value: N) -> Self;
}

/// A value-to-value conversion that consumes the input value. The
/// opposite of [`FromNative`].
///
/// One should avoid implementing `IntoProto` and implement [`FromNative`] instead.
/// Implementing [`FromNative`] automatically provides one with an implementation of `IntoProto`
/// thanks to the blanket implementation in the standard library.
///
/// Prefer using `IntoProto` over [`FromNative`] when specifying trait bounds on a generic function
/// to ensure that types that only implement `IntoProto` can be used as well.
///
///
/// # Generic Implementations
///
/// - [`FromNative`]`<T> for U` implies `IntoProto<U> for T`
pub trait IntoProto<P>: Sized {
    /// Performs the conversion.
    fn into_proto(self) -> P;
}

/// Simple and safe type conversions that may fail in a controlled
/// way under some circumstances. It is the reciprocal of [`TryIntoNative`].
///
/// This is useful when you are doing a type conversion that may
/// trivially succeed but may also need special handling.
/// For example, the proto struct may have an `Option` field that is required
/// in the native side.
///
/// # Generic Implementations
///
/// - `TryFromProto<T> for U` implies [`TryIntoNative`]`<U> for T`
///
/// You should probabily use the derive macro to impl `TryFromProto<P>`
pub trait TryFromProto<P>: Sized {
    /// Performs the conversion.
    fn try_from_proto(value: P) -> Result<Self, ProstConvertError>;
}

/// An attempted conversion that consumes `self`, which may or may not be
/// expensive.
///
/// Library authors should usually not directly implement this trait,
/// but should prefer implementing the [`TryFromProto`] trait, which offers
/// greater flexibility and provides an equivalent `TryIntoNative`
/// implementation for free, thanks to a blanket implementation in this
/// crate.
pub trait TryIntoNative<N>: Sized {
    /// Performs the conversion.
    fn try_into_native(self) -> Result<N, ProstConvertError>;
}

// FIXME:
// - if we want user to impl there custom TryFromProto/FromNative
//   we must add a `dyn Error` Variant. Indeed their convertion function
//   might return something like "`MyCustomTypeParseError". As we don't know
//   what this type will be, we can't add a variant for it.
// - Do we use the Infallible variant?
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum ProstConvertError {
    #[error("prost struct miss a required field")]
    MissingRequiredField,
    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
    #[error("invalid ip address")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("invalid uuid address")]
    UuidEroor(#[from] uuid::Error),
    #[error("int convertion error")]
    TryFromIntError(#[from] TryFromIntError),
    #[error("try to parse a type and failed")]
    TypeParseError(#[from] anyhow::Error),
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

// `TryFromProto` implies `TryIntoNative`.
impl<T, U> TryIntoNative<U> for T
where
    U: TryFromProto<T>,
{
    fn try_into_native(self) -> Result<U, ProstConvertError> {
        U::try_from_proto(self)
    }
}

// `FromNative` implies `IntoProto`.
impl<T, U> IntoProto<U> for T
where
    U: FromNative<T>,
{
    fn into_proto(self) -> U {
        U::from_native(self)
    }
}

// If the field in proto is optional but not the native one, we considered it required.
// If a type T can be created from U, so it can be created from an `Option<U>`.
impl<T, U> TryFromProto<Option<U>> for T
where
    T: TryFromProto<U>,
{
    fn try_from_proto(value: Option<U>) -> Result<Self, ProstConvertError> {
        match value {
            Some(value) => value.try_into_native(),
            None => Err(ProstConvertError::MissingRequiredField),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

// If the native type T can be easily convert into an `Option<T>`
// if required by the proto.
impl<T, U> FromNative<U> for Option<T>
where
    T: FromNative<U>,
{
    fn from_native(value: U) -> Self {
        Some(value.into_proto())
    }
}

// Make this trait usable recursively on vectors.
impl<T, U> TryFromProto<Vec<U>> for Vec<T>
where
    T: TryFromProto<U>,
{
    fn try_from_proto(value: Vec<U>) -> Result<Self, ProstConvertError> {
        let mut native = Vec::with_capacity(value.len());
        for element in value {
            native.push(element.try_into_native()?)
        }
        Ok(native)
    }
}

// Make this trait usable recursively on vectors.
impl<T, U> FromNative<Vec<U>> for Vec<T>
where
    T: FromNative<U>,
{
    fn from_native(value: Vec<U>) -> Self {
        let mut proto = Vec::new();
        for element in value {
            proto.push(element.into_proto());
        }
        proto
    }
}

/// We provide an implementation for all prost scalar value.
/// <https://github.com/tokio-rs/prost#scalar-values>
macro_rules! impl_scalar {
    ( $($t:ty),* ) => {
        $(
            impl TryFromProto<$t> for $t {
                fn try_from_proto(value: $t) -> Result<Self, ProstConvertError> {
                    Ok(value)
                }
            }

            impl FromNative<$t> for $t {
                fn from_native(value: $t) -> Self {
                    value
                }
            }
        )*

    };
}

impl_scalar!(f32, f64, i32, i64, u32, u64, bool, String, Vec<u8>);

macro_rules! impl_map {
    ( $($t:ty),* ) => {

        $(
            impl<T, U> TryFromProto<HashMap<$t, U>> for HashMap<$t, T>
            where
                T: TryFromProto<U>,
            {
                fn try_from_proto(value: HashMap<$t, U>) -> Result<Self, ProstConvertError> {
                    let mut native = HashMap::with_capacity(value.len());
                    for (key, value) in value {
                        native.insert(key, value.try_into_native()?);
                    }
                    Ok(native)
                }
            }

            impl<T, U> FromNative<HashMap<$t, U>> for HashMap<$t, T>
            where
                T: FromNative<U>,
            {
                fn from_native(value: HashMap<$t, U>) -> Self {
                    let mut proto = HashMap::with_capacity(value.len());
                    for (key, value) in value {
                        proto.insert(key, value.into_proto());
                    }
                    proto
                }
            }
        )*
    };
}

// Hashmap key supported by protobuf are only integer or string types
// https://developers.google.com/protocol-buffers/docs/proto3#maps
impl_map!(i32, i64, u32, u64, bool, String);

impl FromNative<PathBuf> for String {
    fn from_native(value: PathBuf) -> Self {
        value.to_string_lossy().into_owned()
    }
}

impl TryFromProto<String> for PathBuf {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

impl FromNative<IpAddr> for String {
    fn from_native(value: IpAddr) -> Self {
        value.to_string()
    }
}

impl TryFromProto<String> for IpAddr {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

impl FromNative<Ipv4Addr> for String {
    fn from_native(value: Ipv4Addr) -> Self {
        value.to_string()
    }
}

impl TryFromProto<String> for Ipv4Addr {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

impl FromNative<Ipv6Addr> for String {
    fn from_native(value: Ipv6Addr) -> Self {
        value.to_string()
    }
}

impl TryFromProto<String> for Ipv6Addr {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

impl FromNative<SocketAddr> for String {
    fn from_native(value: SocketAddr) -> Self {
        value.to_string()
    }
}

impl TryFromProto<String> for SocketAddr {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

impl FromNative<()> for () {
    fn from_native(_: ()) -> Self {}
}

impl TryFromProto<()> for () {
    fn try_from_proto(_: ()) -> Result<Self, ProstConvertError> {
        Ok(())
    }
}

// We can't directly define u16 in proto
impl TryFromProto<u32> for u16 {
    fn try_from_proto(value: u32) -> Result<Self, ProstConvertError> {
        Ok(value.try_into()?)
    }
}

impl FromNative<u16> for u32 {
    fn from_native(value: u16) -> Self {
        value.into()
    }
}

// We can't directly define u8 in proto
impl TryFromProto<u32> for u8 {
    fn try_from_proto(value: u32) -> Result<Self, ProstConvertError> {
        Ok(value.try_into()?)
    }
}

impl FromNative<u8> for u32 {
    fn from_native(value: u8) -> Self {
        value.into()
    }
}

// We can't directly define i16 in proto
impl TryFromProto<i32> for i16 {
    fn try_from_proto(value: i32) -> Result<Self, ProstConvertError> {
        Ok(value.try_into()?)
    }
}

impl FromNative<i16> for i32 {
    fn from_native(value: i16) -> Self {
        value.into()
    }
}

// We can't directly define i8 in proto
impl TryFromProto<i32> for i8 {
    fn try_from_proto(value: i32) -> Result<Self, ProstConvertError> {
        Ok(value.try_into()?)
    }
}

impl FromNative<i8> for i32 {
    fn from_native(value: i8) -> Self {
        value.into()
    }
}

// TODO: This should be under feature flag because it add a depency to uuid which is not mandatory in most use cases.
// Ideally, uuid should have a feature "prost_convert" (like for serde).
impl FromNative<uuid::Uuid> for String {
    fn from_native(value: uuid::Uuid) -> Self {
        value.to_string()
    }
}

impl TryFromProto<String> for uuid::Uuid {
    fn try_from_proto(value: String) -> Result<Self, ProstConvertError> {
        Ok(value.parse()?)
    }
}

// Re-export #[derive(ProstConvert)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls. They
// would need to disable default features and then explicitly re-enable std.

#[cfg(feature = "prost_convert_derive")]
#[doc(hidden)]
pub use prost_convert_derive::ProstConvert;
