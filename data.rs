//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    sync::LazyLock,
    env::args
};

//> HEAD -> SUPER
use super::{
    System,
    argument::Argument,
    error::Error
};

//> HEAD -> RICH_RUST
use rich_rust::{
    console::Console,
    Theme
};

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^ 
//^ DATA
//^ 

//> DATA -> ARGUMENTS
pub static ARGUMENTS: LazyLock<Vec<Argument>> = LazyLock::new(|| {
    let arguments = args().map(Argument::try_from);
    let mut errors = Vec::with_capacity(arguments.len());
    let parsed = arguments.filter_map(|argument| match argument {
        Ok(parsed) => Some(parsed),
        Err(ioerror) => {
            errors.push(ioerror);
            None
        }
    }).collect();
    if !errors.is_empty() {
        System::error(Error::ParsingCommandLineArguments {
            errors: Box::new(NonEmpty::from((errors.remove(0), errors)))
        })
    }
    return parsed;
});

//> DATA -> CONSOLE
pub static CONSOLE: LazyLock<Console> = LazyLock::new(|| {
    return Console::builder().theme(Theme::from_style_definitions(
        [
            ("deprecated", "yellow"),
            ("basic", "gray"),
            ("note", "green"),
            ("error", "bold red"),
            ("cause", "red"),
            ("help", "cyan")
        ], 
        true
    ).unwrap()).highlight(false).build();
});