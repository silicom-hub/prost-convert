use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;
pub mod proto {
    // All protobuf generated struct are pub and their fields also.
    #[derive(PartialEq)]
    pub struct SimpleMessage {
        pub name: String,
    }
}

#[derive(ProstConvert, PartialEq, Debug, Clone)]
#[prost_convert(src = "proto::SimpleMessage")]
pub struct SimpleMessage {
    name: String,
}

fn main() {
    let native = SimpleMessage {
        name: String::from("foo"),
    };
    let proto: proto::SimpleMessage = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
