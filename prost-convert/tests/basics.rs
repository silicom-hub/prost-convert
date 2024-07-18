use prost_convert::{FromNative, IntoProto, ProstConvertError, TryFromProto, TryIntoNative};

macro_rules! try_from_proto_scalar {
    ( $($t:ty),* ) => {
        $(
            let a = <$t>::default();
            let b = <$t>::try_from_proto(a.clone()).unwrap();
            assert_eq!(a, b);
        )*
    };
}

macro_rules! try_into_native_scalar {
    ( $($t:ty),* ) => {
        $(
            let a = <$t>::default();
            let b: $t = a.clone().try_into_native().unwrap();
            assert_eq!(a, b);
        )*
    };
}

macro_rules! try_from_proto_option {
    ( $($t:ty),* ) => {
        $(
            let a = Some(<$t>::default());
            let b = <$t>::try_from_proto(a.clone()).unwrap();
            assert_eq!(a.unwrap(), b);

            let a : Option<$t> = None;
            let b = <$t>::try_from_proto(a.clone());
            assert!(matches!(b.err().unwrap(), ProstConvertError::MissingRequiredField ));
        )*
    };
}

macro_rules! try_into_native_option {
    ( $($t:ty),* ) => {
        $(
            let a = Some(<$t>::default());
            let b: $t = a.clone().try_into_native().unwrap();
            assert_eq!(a.unwrap(), b);

            let a : Option<$t> = None;
            let b : Result<$t, _> = a.clone().try_into_native();

            assert!(matches!(b.err().unwrap(), ProstConvertError::MissingRequiredField ));

        )*
    };
}

#[test]
fn try_from_proto_scalar() {
    try_from_proto_scalar!(f32, f64, i32, i64, u32, u64, bool, String, Vec<u8>);
}

#[test]
fn try_into_native_scalar() {
    try_into_native_scalar!(f32, f64, i32, i64, u32, u64, bool, String, Vec<u8>);
}

#[test]
fn try_from_proto_option() {
    try_from_proto_option!(f32, f64, i32, i64, u32, u64, bool, String, Vec<u8>);
}

#[test]
fn try_into_native_option() {
    try_into_native_option!(f32, f64, i32, i64, u32, u64, bool, String, Vec<u8>);
}

#[test]
fn one_level_nested() {
    pub mod proto {
        #[derive(PartialEq)]
        pub struct SimpleMessage {
            pub name: Option<String>,
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct SimpleMessage {
        name: String,
    }

    // what the derive macro will do:

    impl TryFromProto<proto::SimpleMessage> for SimpleMessage {
        fn try_from_proto(value: proto::SimpleMessage) -> Result<Self, ProstConvertError> {
            Ok(Self {
                name: value.name.try_into_native()?,
            })
        }
    }

    impl From<SimpleMessage> for proto::SimpleMessage {
        fn from(value: SimpleMessage) -> Self {
            Self {
                name: value.name.into(), // tranform it into an option
            }
        }
    }

    // The actual test:

    let native = SimpleMessage {
        name: String::from("foo"),
    };
    let proto: proto::SimpleMessage = native.clone().into();

    assert_eq!(native, proto.try_into_native().unwrap());
}

pub mod proto {
    // All protobuf generated struct are pub and their fields also.
    #[derive(PartialEq)]
    pub struct NestedMessage {
        pub name: String,
    }

    #[derive(PartialEq)]
    pub struct Message {
        pub id: u64,
        pub inner: Option<NestedMessage>,
    }

    #[derive(PartialEq)]
    pub struct TopLevelMessage {
        pub inner: Message,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NestedMessage {
    pub name: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub inner: NestedMessage,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TopLevelMessage {
    pub inner: Message,
}

#[test]
fn try_from_proto_nested() {
    impl TryFromProto<proto::Message> for Message {
        fn try_from_proto(value: proto::Message) -> Result<Self, ProstConvertError> {
            Ok(Self {
                id: value.id.try_into_native()?,
                inner: value.inner.try_into_native()?,
            })
        }
    }

    impl TryFromProto<proto::NestedMessage> for NestedMessage {
        fn try_from_proto(value: proto::NestedMessage) -> Result<Self, ProstConvertError> {
            Ok(Self {
                name: value.name.try_into_native()?,
            })
        }
    }
}

#[test]
fn from_native_nested() {
    impl FromNative<Message> for proto::Message {
        fn from_native(value: Message) -> Self {
            Self {
                id: value.id.into_proto(),
                inner: value.inner.into_proto(),
            }
        }
    }

    impl FromNative<NestedMessage> for proto::NestedMessage {
        fn from_native(value: NestedMessage) -> Self {
            Self {
                name: value.name.into_proto(),
            }
        }
    }
}

#[test]
fn two_level_nested() {
    let native = Message {
        id: 1,
        inner: NestedMessage {
            name: "foo".to_string(),
        },
    };
    let proto: proto::Message = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}
#[test]
fn three_level_nested() {
    impl TryFromProto<proto::TopLevelMessage> for TopLevelMessage {
        fn try_from_proto(value: proto::TopLevelMessage) -> Result<Self, ProstConvertError> {
            Ok(Self {
                inner: value.inner.try_into_native()?,
            })
        }
    }

    impl FromNative<TopLevelMessage> for proto::TopLevelMessage {
        fn from_native(value: TopLevelMessage) -> Self {
            Self {
                inner: value.inner.into_proto(),
            }
        }
    }

    let native = TopLevelMessage {
        inner: Message {
            id: 1,
            inner: NestedMessage {
                name: "foo".to_string(),
            },
        },
    };
    let proto: proto::TopLevelMessage = native.clone().into_proto();

    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn keep_optional_field() {
    pub mod proto {
        #[derive(PartialEq, Debug, Clone)]
        pub struct Message {
            pub field: Option<String>,
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        // I know in the proc macro that this field is optional.
        // If this field is optional, it must be optional in th proto.
        field: Option<String>,
    }

    impl TryFromProto<proto::Message> for Message {
        // changed from classic derive
        fn try_from_proto(value: proto::Message) -> Result<Self, ProstConvertError> {
            Ok(Self {
                field: value
                    .field
                    .map(|field| field.try_into_native())
                    .transpose()?,
            })
        }
    }

    impl FromNative<Message> for proto::Message {
        // changed from classic derive
        fn from_native(value: Message) -> Self {
            Self {
                field: value.field.map(|field| field.into_proto()),
            }
        }
    }

    let native = Message {
        field: Some(String::from("foo")),
    };

    dbg!(&native);

    let proto: proto::Message = native.clone().into_proto();
    dbg!(&proto);

    assert_eq!(native, proto.try_into_native().unwrap());
}

#[test]
fn keep_optional_user_defined_field() {
    pub mod proto {
        #[derive(PartialEq, Debug, Clone)]
        pub struct Message {
            pub field: Option<Nested>,
        }

        #[derive(PartialEq, Debug, Clone)]
        pub struct Nested {
            pub id: i32,
        }
    }

    // I know in the proc macro that the field is optional.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        field: Option<Nested>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Nested {
        pub id: i32,
    }

    impl TryFromProto<proto::Message> for Message {
        // changed from classic derive
        fn try_from_proto(value: proto::Message) -> Result<Self, ProstConvertError> {
            Ok(Self {
                field: value
                    .field
                    .map(|field| field.try_into_native())
                    .transpose()?,
            })
        }
    }

    impl FromNative<Message> for proto::Message {
        // changed from classic derive
        fn from_native(value: Message) -> Self {
            Self {
                field: value.field.map(|field| field.into_proto()),
            }
        }
    }

    impl TryFromProto<proto::Nested> for Nested {
        fn try_from_proto(value: proto::Nested) -> Result<Self, ProstConvertError> {
            Ok(Self {
                id: value.id.try_into_native()?,
            })
        }
    }

    impl FromNative<Nested> for proto::Nested {
        fn from_native(value: Nested) -> Self {
            Self {
                id: value.id.into_proto(),
            }
        }
    }

    let native = Message {
        field: Some(Nested { id: 2 }),
    };
    let proto: proto::Message = native.clone().into_proto();
    assert_eq!(native, proto.try_into_native().unwrap());
}
