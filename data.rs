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
    argument::Argument
};

//> HEAD -> RICH_RUST
use rich_rust::console::Console;


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
    if !errors.is_empty() {System::critical(errors)}
    return parsed;
});

//> DATA -> CONSOLE
pub static CONSOLE: LazyLock<Console> = LazyLock::new(|| {
    return Console::builder().highlight(false).build();
});