#![feature(once_cell, min_specialization)]
#![warn(rust_2018_idioms)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![doc(html_root_url = "https://stackdoubleflow.github.io/quest-hook-rs/quest_hook/libil2cpp")]

mod array;
mod class;
mod exception;
mod field_info;
mod method_info;
mod object;
mod parameter_info;
pub mod raw;
mod string;
mod ty;
mod typecheck;

pub use array::Il2CppArray;
pub use class::Il2CppClass;
pub use exception::Il2CppException;
pub use field_info::FieldInfo;
pub use method_info::MethodInfo;
pub use object::Il2CppObject;
pub use parameter_info::ParameterInfo;
pub use raw::WrapRaw;
pub use string::Il2CppString;
pub use ty::{Builtin, Il2CppType};
pub use typecheck::callee::{Parameter, Parameters, Return as CalleeReturn, This as CalleeThis};
pub use typecheck::caller::{Argument, Arguments, Return, This};
pub use typecheck::ty::Type;

// Maybe I have OCD
pub use {Parameter as CalleeArgument, Parameters as CalleeArguments};
