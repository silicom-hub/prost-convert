// Test extracted from Dtolnay workshop <https://github.com/dtolnay/proc-macro-workshop>
//
// Does your macro still work if some of the standard library prelude item names
// mean something different in the caller's code?
//
// It may seem unreasonable to consider this case, but it does arise in
// practice. Most commonly for Result, where crates sometimes use a Result type
// alias with a single type parameter which assumes their crate's error type.
// Such a type alias would break macro-generated code that expects Result to
// have two type parameters. As another example, Hyper 0.10 used to define
// hyper::Ok as a re-export of hyper::status::StatusCode::Ok which is totally
// different from Result::Ok. This caused problems in code doing `use hyper::*`
// together with macro-generated code referring to Ok.
//
// Generally all macros (procedural as well as macro_rules) designed to be used
// by other people should refer to every single thing in their expanded code
// through an absolute path, such as std::result::Result.

use prost_convert_derive::ProstConvert;

#[allow(dead_code)]
type Option = ();
#[allow(dead_code)]
type Some = ();
#[allow(dead_code)]
type None = ();
#[allow(dead_code)]
type Result = ();
#[allow(dead_code)]
type Box = ();
#[allow(dead_code)]
type Vec = ();

pub mod proto {
    // All protobuf generated struct are pub and their fields also.
    pub struct SimpleMessage {
        pub name: String,
    }
}

#[derive(ProstConvert)]
#[prost_convert(src = "proto::SimpleMessage")]
pub struct SimpleMessage {
    name: String,
}

fn main() {}
