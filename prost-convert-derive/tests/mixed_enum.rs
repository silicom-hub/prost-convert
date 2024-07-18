use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    #[derive(PartialEq)]
    pub enum TunnelType {
        ImplantTunnel(()),
        Forward(i32),
    }

    #[derive(PartialEq)]
    pub enum ForwardTunnelTask {
        Socks5Proxy = 0,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::TunnelType")]
enum TunnelType {
    ImplantTunnel(()), // Sad workaround, ideally we shouldn't define newtype.
    Forward(ForwardTunnelTask),
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::ForwardTunnelTask")]
enum ForwardTunnelTask {
    Socks5Proxy,
}

#[test]
fn it_works() {
    let native = TunnelType::Forward(ForwardTunnelTask::Socks5Proxy);
    let proto: proto::TunnelType = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
