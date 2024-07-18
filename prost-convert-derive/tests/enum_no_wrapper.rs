use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    #[derive(PartialEq)]
    pub enum Foo {
        Foo1,
        Foo2,
    }
    #[derive(PartialEq)]
    pub struct Baz {
        pub id: i32,
    }

    #[derive(PartialEq)]
    pub struct Message {
        pub foo: Option<Foo>,
        pub bar: String,
        pub baz: Option<Baz>,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Foo")]
enum Foo {
    Foo1,
    Foo2,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Message")]
struct Message {
    foo: Foo,
    bar: String,
    baz: Baz,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Baz")]
struct Baz {
    id: i32,
}

#[test]
fn enum_no_wrapper() {
    let native = Message {
        foo: Foo::Foo1,
        bar: "hello".to_string(),
        baz: Baz { id: 2 },
    };
    let proto: proto::Message = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
