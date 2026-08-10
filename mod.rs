//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(core_io)]
#![feature(generic_const_exprs)]
#![feature(try_blocks)]
#![feature(default_field_values)]

//> HEAD -> MODULES
mod argument;
mod clitype;
mod data;
mod descriptor;
mod handling;
mod ioerror;
mod metadata;
mod openmode;
mod path;
mod problem;

//> HEAD -> STD
use std::{
    panic::set_hook,
    path::PathBuf
};

//> HEAD -> CORE
use core::fmt::{
    Debug,
    Display
};

//> HEAD -> IOERROR
pub use ioerror::IoError;

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> PROBLEM
use problem::Problem;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> CLITYPE
pub use clitype::CliType;

//> HEAD -> PATH
use path::Path;

//> HEAD -> OPENMODE
pub use openmode::{
    Overwrite,
    Read,
    Append
};

//> HEAD -> DATA
use data::{
    ARGUMENTS,
    CONSOLE
};

//> HEAD -> HANDLING
pub use handling::Handling;


//^
//^ SYSTEM
//^

//> SYSTEM -> ENUM
pub enum System {}

//> SYSTEM -> IMPLEMENTATION
impl System {
    pub fn arguments() -> &'static [Argument] {return ARGUMENTS.as_slice()}
    pub fn path(
        filename: impl Into<PathBuf>
    ) -> Path {return Path::from(filename.into())}
    pub fn warning(object: impl Into<Issue>) -> () {
        CONSOLE.print(&Problem {
            issue: Into::<Issue>::into(object),
            severity: Some(false)
        }.to_string());
    }
    pub fn error(object: impl Into<Issue>) -> () {
        CONSOLE.print(&Problem {
            issue: Into::<Issue>::into(object),
            severity: Some(true)
        }.to_string());
    }
    pub fn critical(iterator: impl IntoIterator<Item = impl Into<Issue>>) -> ! {
        for object in iterator {CONSOLE.print(&Problem {
            issue: Into::<Issue>::into(object),
            severity: None
        }.to_string())}
        set_hook(Box::new(|_| ()));
        panic!();
    }
    pub fn expect<Type>(result: Result<Type, impl Into<Issue>>) -> Type {return match result {
        Ok(value) => value,
        Err(error) => Self::critical([error])
    }}
    pub fn print(value: impl Display) -> () {CONSOLE.print(&value.to_string())}
    pub fn debug(value: impl Debug, raw: bool) -> () {
        CONSOLE.print(&if raw {format!("{value:?}")} else {format!("{value:#?}")});
    }
}
