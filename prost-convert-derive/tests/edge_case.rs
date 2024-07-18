use std::net::{Ipv4Addr, Ipv6Addr};

use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    pub struct NetworkInterface {
        pub name: Option<String>,
        pub mac_addr: Option<String>,
        pub addr: Option<Addr>,
    }

    pub enum Addr {
        V4(V4IfAddr),
        V6(V6IfAddr),
    }

    pub struct V6IfAddr {
        pub ip: String,
        pub broadcast: Option<String>,
        pub netmask: Option<String>,
    }

    pub struct V4IfAddr {
        pub ip: String,
        pub broadcast: Option<String>,
        pub netmask: Option<String>,
    }
}

#[derive(Clone, PartialEq, Eq, Default, ProstConvert, Debug)]
#[prost_convert(src = "proto::NetworkInterface")]
pub struct NetworkInterface {
    /// Interface's name.
    pub name: Option<String>,
    /// Interface's address.
    pub addr: Option<Addr>,
    /// MAC Address.
    pub mac_addr: Option<String>,
}

#[derive(Clone, PartialEq, Eq, ProstConvert, Debug)]
#[prost_convert(src = "proto::Addr")]
pub enum Addr {
    V4(V4IfAddr),
    V6(V6IfAddr),
}

pub type Netmask<T> = Option<T>;

#[derive(Clone, PartialEq, Eq, ProstConvert, Debug)]
#[prost_convert(src = "proto::V4IfAddr")]
pub struct V4IfAddr {
    pub ip: Ipv4Addr,
    pub broadcast: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
}

#[derive(Clone, PartialEq, Eq, ProstConvert, Debug)]
#[prost_convert(src = "proto::V6IfAddr")]
pub struct V6IfAddr {
    pub ip: Ipv6Addr,
    pub broadcast: Option<Ipv6Addr>,
    pub netmask: Option<Ipv6Addr>,
}

#[test]
fn it_works() {
    let native = NetworkInterface {
        name: None,
        addr: None,
        mac_addr: None,
    };
    let proto: proto::NetworkInterface = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
