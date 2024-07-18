use prost_convert::{FromNative, IntoProto, TryFromProto, TryIntoNative};
use prost_convert_derive::ProstConvert;
use uuid::Uuid;

pub mod proto {
    #[derive(PartialEq)]
    pub struct Message {
        pub id: String,
    }

    #[derive(PartialEq)]
    pub struct Log {
        pub log: String,
    }

    #[derive(PartialEq)]
    pub struct ModuleLogs {
        pub logs: Vec<Log>, // inner of the new type.
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Id(Uuid);

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Message")]
struct Message {
    id: Id,
}

// Here the macro get:
// - "ModuleLogs" :  ident of the native struct
// - "proto::ModuleLogs" : ident of the proto struct
// - one field of this struct is a newtype.

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::ModuleLogs")]
pub struct ModuleLogs {
    logs: LogCollection,
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Log")]
struct Log {
    log: String,
}

impl TryFromProto<String> for Id {
    fn try_from_proto(value: String) -> Result<Self, prost_convert::ProstConvertError> {
        Ok(Self(value.try_into_native()?))
    }
}

impl FromNative<Id> for String {
    fn from_native(value: Id) -> Self {
        value.0.into_proto()
    }
}

// FIME: this should be optained from // #[derive(ProstConvert)]
// - The macro can't deduce what is the proto equivalent of the wrapped type.
// - The macro only know the wrapper ident and that a new type is used.
// The wrapped path doesn't have always a valid path, ex "std::vec::Vec<proto::Log>"
// So we must define a new attribute that tale a `syn::Type`

#[derive(PartialEq, Debug, Clone)]
// #[derive(ProstConvert)]
// #[prost_convert(src = "std::vec::Vec<proto::Log>")]

struct LogCollection(Vec<Log>);

// To generate this we need :
// - "Vec<proto::Log>" -> this info is inside the proto file so we can't deduce it.
// - "LogCollection" -> OK
// - info that's a new type is used -> OK

// Simple case
impl TryFromProto<Vec<proto::Log>> for LogCollection {
    fn try_from_proto(value: Vec<proto::Log>) -> Result<Self, prost_convert::ProstConvertError> {
        Ok(Self(value.try_into_native()?))
    }
}

impl FromNative<LogCollection> for Vec<proto::Log> {
    fn from_native(value: LogCollection) -> Self {
        value.0.into_proto()
    }
}

#[test]
fn simple_case() {
    let _: std::vec::Vec<proto::Log> = Vec::new();
    let native = ModuleLogs {
        logs: LogCollection(vec![Log {
            log: String::from("foo"),
        }]),
    };
    let proto: proto::ModuleLogs = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn it_works() {
    let native = Message {
        id: Id(Uuid::new_v4()),
    };
    let proto: proto::Message = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
