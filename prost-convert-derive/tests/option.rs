// if the native field is also an `Option<T>`, the `Option` must be kept.

use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {

    #[derive(PartialEq)]
    pub enum Foo {
        Foo1,
        Foo2,
    }
    #[derive(PartialEq)]
    pub struct Message {
        pub field: Option<String>,
        pub optional_enum: Option<Foo>,
    }

    #[derive(PartialEq)]
    pub struct Message2 {
        pub field: Option<Nested>,
        pub optional_enum: Option<Foo>,
    }

    #[derive(PartialEq)]
    pub struct Nested {
        pub id: i32,
    }

    #[derive(PartialEq)]
    pub struct SimpleMessage {
        pub name: Option<String>,
    }
}

#[derive(ProstConvert, PartialEq, Debug, Clone)]
#[prost_convert(src = "proto::SimpleMessage")]
pub struct SimpleMessage {
    name: String,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Foo")]
pub enum Foo {
    Foo1,
    Foo2,
}

#[derive(Debug, Clone, PartialEq, ProstConvert)]
#[prost_convert(src = "proto::Message")]
pub struct Message {
    field: Option<String>,
    optional_enum: Option<Foo>,
}

#[derive(Debug, Clone, PartialEq, ProstConvert)]
#[prost_convert(src = "proto::Message2")]
pub struct Message2 {
    field: Option<Nested>,
    optional_enum: Option<Foo>,
}

#[derive(Debug, Clone, PartialEq, ProstConvert)]
#[prost_convert(src = "proto::Nested")]
pub struct Nested {
    pub id: i32,
}

#[test]
fn keep_optional() {
    let native = Message {
        field: Some(String::from("foo")),
        optional_enum: Some(Foo::Foo1),
    };
    let proto: proto::Message = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn keep_optional_nested() {
    let native = Message2 {
        field: Some(Nested { id: 2 }),
        optional_enum: Some(Foo::Foo1),
    };

    let proto: proto::Message2 = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn unwanted_option() {
    let native = SimpleMessage {
        name: String::from("foo"),
    };
    let proto: proto::SimpleMessage = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
