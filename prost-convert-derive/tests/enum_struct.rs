use prost_convert::{FromNative, IntoProto, TryFromProto, TryIntoNative};
use prost_convert_derive::ProstConvert;

pub mod proto {
    pub struct New {
        pub source: String,
        pub destination: String,
    }

    pub struct Remove {
        pub source: String,
        pub destination: String,
    }

    pub struct Lost {
        pub id: String,
    }

    pub enum Route {
        New(New),
        Remove(Remove),
        Lost(Lost),
    }

    pub struct Message {
        pub route: Route,
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Route {
    New { source: String, destination: String },
    Remove { source: String, destination: String },
    Lost { id: String },
}

// This can be generated from the macro
impl TryFromProto<proto::Route> for Route {
    fn try_from_proto(value: proto::Route) -> Result<Self, prost_convert::ProstConvertError> {
        let res = match value {
            proto::Route::New(new) => Self::New {
                source: new.source,
                destination: new.destination,
            },
            proto::Route::Remove(remove) => Self::Remove {
                source: remove.source,
                destination: remove.destination,
            },
            proto::Route::Lost(lost) => Self::Lost { id: lost.id },
        };
        Ok(res)
    }
}

// We currently can't generate it through the macro. Indeed, we don't have the path to the struct/enum inside
// the new type (for instance in the example below `proto::New`, `proto::Remove` and `proto::Lost`) and if found no way to guess it.
// We can use a convention (for instance the new type should be the same name and in the same module inside the proto)
// because it won't work with foreign *.proto*. The solution could be to annotate each variant of the native enum with the path
// of the corresponding proto enum/struct.
impl FromNative<Route> for proto::Route {
    fn from_native(value: Route) -> Self {
        match value {
            Route::New {
                source,
                destination,
            } => Self::New(proto::New {
                source: source.into_proto(),
                destination: destination.into_proto(),
            }),
            Route::Remove {
                source,
                destination,
            } => Self::Remove(proto::Remove {
                destination: destination.into_proto(),
                source: source.into_proto(),
            }),
            Route::Lost { id } => Self::Lost(proto::Lost {
                id: id.into_proto(),
            }),
        }
    }
}

#[derive(PartialEq, Debug, Clone, ProstConvert)]
#[prost_convert(src = "proto::Message")]
struct Message {
    route: Route,
}

#[test]
fn it_works() {
    let native = Message {
        route: Route::Lost {
            id: String::from("hello"),
        },
    };
    let proto: proto::Message = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
