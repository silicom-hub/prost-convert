use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    #[derive(PartialEq)]
    pub enum Foo {
        Foo1,
        Foo2,
    }
    #[derive(PartialEq)]
    pub struct FooWrapper {
        pub foo: Foo,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Foo", wrapper = "proto::FooWrapper")]
enum Foo {
    Foo1,
    Foo2,
}

#[test]
fn enum_wrapper() {
    let native = Foo::Foo1;
    let proto: proto::FooWrapper = native.clone().into_proto();
    // works also
    let _: proto::Foo = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
