use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {

    #[derive(PartialEq)]
    pub struct OrderOutputWrapper {
        pub order_output: Option<OrderOutput>,
    }

    #[derive(PartialEq)]
    pub enum OrderOutput {
        Result(String),
        Log(String),
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::OrderOutput")]
enum OrderOutput {
    Result(String),
    Log(String),
}

#[test]
fn new_type() {
    let native = OrderOutput::Log("Hello".to_string());
    let proto: proto::OrderOutput = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
