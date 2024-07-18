use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;
use std::net::{IpAddr, Ipv4Addr};

pub mod proto {
    pub struct Message {
        // protobuf does not provide an ip adresse type. string is often used for that,
        // notably in google.
        pub ip: String,
    }

    pub struct Interface {
        pub ip: Option<String>,
    }

    pub struct InterfaceV4 {
        pub ip: Option<String>,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Message")]
struct Message {
    ip: IpAddr,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Interface")]
struct Interface {
    ip: Option<IpAddr>,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::InterfaceV4")]
struct InterfaceV4 {
    ip: Option<Ipv4Addr>,
}

#[test]
fn ip_adress() {
    let native = Message {
        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    };
    let proto: proto::Message = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
