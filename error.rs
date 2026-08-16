//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> CORE
use core::io::Error as IoError;

//> HEAD -> STD
use std::{
    string::FromUtf8Error,
    num::{
        ParseIntError, 
        ParseFloatError
    },
    path::PathBuf
};

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^
//^ ERROR
//^

//> ERROR -> ENUM
#[derive(Debug)]
pub enum Error<'valid> {
    OpeningFile {
        ioerror: IoError,
        name: PathBuf
    },
    ReadingMetadata {
        ioerror: IoError
    },
    ReadingFile {
        ioerror: IoError
    },
    WritingToFile {
        ioerror: IoError
    },
    EncodingUnicode {
        ioerror: FromUtf8Error
    },
    ParsingSetting {
        value: String,
        numbererror: ParseIntError,
        floaterror: ParseFloatError
    },
    ParsingArgument {
        argument: String
    },
    DeterminingPathExists {
        path: &'valid PathBuf,
        ioerror: IoError
    },
    ParsingCommandLineArguments {
        errors: Box<NonEmpty<Error<'valid>>>
    }
}

//> ERROR -> INTO ISSUE
impl<'valid> Into<Issue> for Error<'valid> {
    fn into(self) -> Issue {return match self {
        Error::OpeningFile {ioerror, name} => Issue {
            name: "failed to open file",
            sections: Vec::from([
                Section::Help(format!(
                    "you might want to create it first{}",
                    match name.as_os_str().to_str() {
                        None => String::default(),
                        Some(string) => format!(": `touch {string}`")
                    }
                )),
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::ReadingMetadata {ioerror} => Issue {
            name: "failed to read file metadata",
            sections: Vec::from([
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::ReadingFile {ioerror} => Issue {
            name: "failed to read file",
            sections: Vec::from([
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::WritingToFile {ioerror} => Issue {
            name: "failed to write to file",
            sections: Vec::from([
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::EncodingUnicode {ioerror} => Issue {
            name: "failed to encode file to UTF-8",
            sections: Vec::from([
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::ParsingSetting {value, numbererror, floaterror} => Issue {
            name: "failed to parse setting value",
            description: Some(format!("failed to parse {value:?}")),
            sections: Vec::from([
                Section::Traceback(numbererror.to_string()),
                Section::Traceback(floaterror.to_string())
            ]),
            ..
        },
        Error::ParsingArgument {argument} => Issue {
            name: "failed to parse argument for command line",
            description: Some(format!("failed to parse argument {argument:?}")),
            ..
        },
        Error::DeterminingPathExists {path, ioerror} => Issue {
            name: "failed to check if path exists",
            description: Some(format!(
                "couldn't verify {} exists", 
                path.to_str().map(|name| {
                    format!("{name:?}")
                }).unwrap_or(String::from("file"))
            )),
            sections: Vec::from([
                Section::Traceback(ioerror.to_string())
            ]),
            ..
        },
        Error::ParsingCommandLineArguments {errors} => Issue {
            name: "failed to parse CLI arguments",
            description: Some(format!(
                "failed to parse {} argument{}",
                errors.len(),
                if errors.len() != 1 {"s"} else {""}
            )),
            sections: {
                errors.into_iter().map(|error| {
                    Section::Child(error.into())
                }).collect()
            },
            ..
        }
    }}
}
