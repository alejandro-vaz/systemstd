//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> SYSTEMSTD
use systemstd::{
    Argument, 
    CliType, 
    Error, 
    System,
    Read,
    Severity,
    Handling
};

//> HEAD -> STD
use std::assert_matches;

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Span,
    Section
};


//^
//^ TESTS
//^

//> TESTS -> PRINT
#[test]
fn print() -> () {
    System::print("value. This **is** Markdown boys.\n\n## big title", true);
    System::debug("hello", false);
}

//> TESTS -> CLI
#[test]
fn cli() -> () {
    let _arguments = System::arguments();
}

//> TESTS -> ARGUMENTS
#[test]
fn arguments() -> () {
    assert_matches!("myexec.exect".to_string().try_into(), Ok(Argument::Path {..}));
    assert_matches!("rm".to_string().try_into(), Ok(Argument::Target {..}));
    assert_matches!("-rf".to_string().try_into(), Ok(Argument::Alias {..}));
    assert_matches!("--please".to_string().try_into(), Ok(Argument::Flag {..}));
    assert_matches!("--opt=3".to_string().try_into(), Ok(Argument::Setting {
        value: CliType::Integer {
            value: 3
        },
        ..
    }));
    assert_matches!(TryInto::<Argument>::try_into("--key=impossible".to_string()), Err(
        Error::ParsingSetting {..}
    ));
    assert_matches!(TryInto::<Argument>::try_into("&".to_string()), Err(
        Error::ParsingArgument {..}
    ));
}

//> TESTS -> READ
#[test]
fn read() -> () {assert_eq!(
    System::path("Cargo.toml").file::<Read>(
        Handling::AssumeExists
    ).unwrap().read_bytes().unwrap().into_iter().next(),
    Some(b'[')
)}

//> TESTS -> NESTED
#[test]
fn nested() -> () {
    pub enum VeryImportant {} impl Severity for VeryImportant {
        type Then = ();
        const COLOR: &'static str = "red";
        const SYMBOL: char = '#';
        fn done() -> Self::Then {}
    }
    let first = Issue {
        name: "this one is nested",
        description: Some(format!("checking format is alright")),
        sections: Vec::from([
            Section::Traceback(format!("this issue comes from hell"))
        ]),
        ..
    };
    let second = Issue {
        name: "hello",
        description: Some(String::from("description!!!")),
        sections: Vec::from([
            Section::Child(first),
            Section::Help(format!("die")),
            Section::Note(format!("please"))
        ]),
        ..
    };
    System::error::<VeryImportant>(second);
    let third = Issue {
        name: "third example",
        deprecation: Some(format!("please don't use this")),
        description: Some(String::from("see some code")),
        sections: Vec::from([
            Section::Code {
                code: String::from("println!(\"hello\")"),
                language: Some("rust"),
                line: Some(1),
                message: Some(String::from("this line prints hello to the console")),
                span: Some(Span::RangeFull(..))
            }
        ]),
        ..
    };
    System::error::<VeryImportant>(third);
}

//> TESTS -> EMPTY
#[test]
fn empty() -> () {
    pub enum VeryImportant {} impl Severity for VeryImportant {
        type Then = ();
        const COLOR: &'static str = "red";
        const SYMBOL: char = '#';
        fn done() -> Self::Then {}
    }
    let empty = Issue {
        name: "empty??",
        ..
    };
    System::error::<VeryImportant>(empty);
}

//> TESTS -> BADARG
#[test]
#[should_panic]
fn badarg() -> () {
    System::expect::<System, _>(Argument::try_from(format!("&&&&nonsense")));
}

//> TESTS -> BADSET
#[test]
#[should_panic]
fn badset() -> () {
    System::expect::<System, _>(Argument::try_from(format!("--a=22s")));
}