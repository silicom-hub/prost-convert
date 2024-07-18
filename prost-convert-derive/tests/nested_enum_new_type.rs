use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {

    #[derive(PartialEq)]
    pub struct EventResponse {
        pub event: Option<Event>,
    }

    #[derive(PartialEq)]
    pub enum Event {
        ModuleExecutionOutput(OrderOutputWrapper),
        NewImplantRegistration(ImplantRegistration),
    }

    #[derive(PartialEq)]
    pub struct OrderOutputWrapper {
        pub order_output: Option<OrderOutput>,
    }

    #[derive(PartialEq)]
    pub enum OrderOutput {
        Result(String),
        Log(String),
    }

    #[derive(PartialEq)]
    pub struct ImplantRegistration {
        pub id: u32,
    }
}

// Native struct

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Event", wrapper = "proto::EventResponse")]
enum Event {
    /// Order output Event.
    ModuleExecutionOutput(OrderOutput),
    /// New implant registration info.
    NewImplantRegistration(ImplantRegistration),
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::OrderOutput", wrapper = "proto::OrderOutputWrapper")]
enum OrderOutput {
    Result(String),
    Log(String),
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::ImplantRegistration")]
struct ImplantRegistration {
    id: u32,
}

#[test]
fn it_works() {
    let native = Event::ModuleExecutionOutput(OrderOutput::Log(String::from("Hello")));
    let proto: proto::EventResponse = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
