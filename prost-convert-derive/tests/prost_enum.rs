// When using protobuf enum inside other message, prost transform
// them into i32.

use prost_convert::{IntoProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {

    // Prost add explicit discriminant into unit enum and use those
    // to generate conversion function from i32;
    #[derive(PartialEq)]
    pub enum OperatingSystem {
        Windows = 0,
        Linux = 1,
    }

    #[derive(PartialEq)]
    pub struct Computer {
        pub os: i32,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::OperatingSystem")]
pub enum OperatingSystem {
    Windows,
    Linux,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Computer")]
pub struct Computer {
    pub os: OperatingSystem,
}

#[test]
fn it_works() {
    let native = Computer {
        os: OperatingSystem::Linux,
    };
    let proto: proto::Computer = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
