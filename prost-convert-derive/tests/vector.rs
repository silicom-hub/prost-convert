use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    #[derive(PartialEq)]
    pub struct Message {
        pub nesteds: Vec<Nested>,
    }

    #[derive(PartialEq)]
    pub struct SimpleMessage {
        pub nesteds: Vec<String>,
    }

    #[derive(PartialEq)]
    pub struct Nested {
        pub field: String,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::SimpleMessage")]
struct SimpleMessage {
    nesteds: Vec<String>,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Message")]
struct Message {
    nesteds: Vec<Nested>,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Nested")]
struct Nested {
    field: String,
}

#[test]
fn simple() {
    let native = SimpleMessage {
        nesteds: vec![String::from("hello"), String::from("world")],
    };
    let proto: proto::SimpleMessage = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn nested() {
    let native = Message {
        nesteds: vec![
            Nested {
                field: String::from("hello"),
            },
            Nested {
                field: String::from("world"),
            },
        ],
    };
    let proto: proto::Message = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
