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
#![feature(default_field_values)]
#![feature(never_type)]

//> HEAD -> MODULES
mod argument;
mod clitype;
mod data;
mod descriptor;
mod display;
mod error;
mod handling;
mod metadata;
mod openmode;
mod path;
mod severity;

//> HEAD -> STD
use std::{
    path::PathBuf,
    panic::set_hook
};

//> HEAD -> CORE
use core::fmt::Debug;

//> HEAD -> ERROR
pub use error::Error;

//> HEAD -> ISSUING
use issuing::Issue;

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

//> HEAD -> RICH_RUST
use rich_rust::prelude::Markdown;

//> HEAD -> SEVERITY
pub use severity::Severity;

//> HEAD -> DISPLAY
use display::display;


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
    pub fn error<Mode: Severity>(object: impl Into<Issue>) -> Mode::Then {
        CONSOLE.print(&display::<Mode>(object.into()));
        return Mode::done();
    }
    pub fn expect<Mode: Severity<Then = !>, Type>(
        result: Result<Type, impl Into<Issue>>
    ) -> Type {return match result {
        Ok(value) => value,
        Err(error) => Self::error::<Mode>(error)
    }}
    pub fn print(string: &str, markdown: bool) -> () {if markdown {
        CONSOLE.print_renderable(&Markdown::new(string))
    } else {
        CONSOLE.print_plain(string);
    }}
    pub fn debug(value: impl Debug, raw: bool) -> () {
        CONSOLE.print_plain(&if raw {format!("{value:?}")} else {format!("{value:#?}")});
    }
}

//> SYSTEM -> SEVERITY
impl Severity for System {
    type Then = !;
    const COLOR: &'static str = "red";
    const SYMBOL: char = '@';
    fn done() -> Self::Then {
        set_hook(Box::new(|_| ()));
        panic!();
    }
}