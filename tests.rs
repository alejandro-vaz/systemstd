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
    let first = Issue {
        name: "this one is nested",
        sections: Vec::from([
            Section::Note(format!("checking format is alright")),
            Section::Cause(format!("this issue comes from hell"))
        ]),
        ..
    };
    let second = Issue {
        name: "hello",
        sections: Vec::from([
            Section::Child(first),
            Section::Help(format!("die")),
            Section::Note(format!("please"))
        ]),
        ..
    };
    System::error(second);
    let third = Issue {
        name: "third example",
        sections: Vec::from([
            Section::Deprecated(format!("please don't use this")),
            Section::Note(format!("see some code")),
            Section::Code {
                extends: Box::new(Section::Note(format!(
                    "this line prints hello to the console"
                ))),
                code: String::from("println!(\"hello\")"),
                language: Some("rust"),
                line: Some(1),
                ..
            }
        ]),
        ..
    };
    System::error(third);
}

//> TESTS -> EMPTY
#[test]
fn empty() -> () {
    #[derive(Debug)]
    pub enum VeryImportant {A} 
    impl Into<Issue> for VeryImportant {
        fn into(self) -> Issue {Issue {
        name: "empty??",
        ..
    }}
    }
    impl Severity for VeryImportant {
        type Then = ();
        fn done() -> Self::Then {}
    }
    System::error::<VeryImportant>(VeryImportant::A);
}

//> TESTS -> BADARG
#[test]
#[should_panic]
fn badarg() -> () {
    System::expect(Argument::try_from(format!("&&&&nonsense")));
}

//> TESTS -> BADSET
#[test]
#[should_panic]
fn badset() -> () {
    System::expect(Argument::try_from(format!("--a=22s")));
}