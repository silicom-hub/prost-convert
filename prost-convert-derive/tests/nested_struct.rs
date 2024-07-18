use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;
pub mod proto {
    // All protobuf generated struct are pub and their fields also.
    #[derive(PartialEq)]
    pub struct NestedMessage {
        pub name: String,
    }

    #[derive(PartialEq)]
    pub struct Message {
        pub id: u64,
        pub inner: Option<NestedMessage>,
    }
}

#[derive(ProstConvert, PartialEq, Debug, Clone)]
#[prost_convert(src = "proto::NestedMessage")]
pub struct NestedMessage {
    pub name: String,
}

#[derive(ProstConvert, PartialEq, Debug, Clone)]
#[prost_convert(src = "proto::Message")]
pub struct Message {
    pub id: u64,
    pub inner: NestedMessage,
}

#[test]
fn nested_struct() {
    let native = Message {
        id: 1,
        inner: NestedMessage {
            name: "foo".to_string(),
        },
    };
    let proto: proto::Message = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
