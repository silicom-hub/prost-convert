use prost_convert::{IntoProto, TryIntoNative};
use std::collections::HashMap;

use prost_convert_derive::ProstConvert;

pub mod proto {
    use std::collections::HashMap;
    #[derive(PartialEq)]
    pub struct PayloadConfigs {
        pub payload_configs: HashMap<String, PayloadConfig>,
    }
    #[derive(PartialEq)]
    pub struct PayloadConfig {
        pub os: i32,
        pub arch: i32,
        pub payload_parameters: Option<PayloadParameters>,
    }
    #[derive(PartialEq)]
    pub enum PayloadParameters {
        Implant(ImplantParameters),
    }
    #[derive(PartialEq)]
    pub struct ImplantParameters {
        pub c2_location: String,
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::PayloadConfigs")]
pub struct PayloadConfigs {
    pub payload_configs: HashMap<String, PayloadConfig>,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::PayloadConfig")]
pub struct PayloadConfig {
    pub os: i32,
    pub arch: i32,
    pub payload_parameters: PayloadParameters,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::PayloadParameters")]
pub enum PayloadParameters {
    Implant(ImplantParameters),
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::ImplantParameters")]
pub struct ImplantParameters {
    pub c2_location: String,
}

#[test]
fn basic_hashmap() {
    let native = PayloadConfigs {
        payload_configs: HashMap::from([(
            String::from("payload1"),
            PayloadConfig {
                os: 1,
                arch: 2,
                payload_parameters: PayloadParameters::Implant(ImplantParameters {
                    c2_location: String::from("here"),
                }),
            },
        )]),
    };
    let proto: proto::PayloadConfigs = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}

// optional map is not supported in proto3. For now, we just use
// normal hasmap that can be empty.
// https://github.com/protocolbuffers/protobuf/issues/8419

// Sometimes the key also need to recursively impl TryIntoNative/FromProto
// bust we don't curently support that case.
// In theorie if the key can be converted with TryIntoNative/FromProto into a supported
// protobuf hashmap key (i32, i64, u32, u64, bool, String) we should support it
#[test]
fn different_key() {
    // pub mod proto {
    //     use std::collections::HashMap;

    //     pub struct Graph {
    //         nodes: HashMap<u64, String>,
    //     }
    // }

    // pub struct Graph {
    //     nodes: HashMap<Id, String>,
    // }

    // pub struct Id(u64);
}
