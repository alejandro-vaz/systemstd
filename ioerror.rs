//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> CORE
use core::io::Error;

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
//^ STDIOERROR
//^

//> STDIOERROR -> ENUM
#[derive(Debug)]
pub enum IoError<'valid> {
    OpeningFile {
        error: Error,
        name: PathBuf
    },
    ReadingMetadata {
        error: Error
    },
    ReadingFile {
        error: Error
    },
    WritingToFile {
        error: Error
    },
    EncodingUnicode {
        error: FromUtf8Error
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
        error: Error
    },
    ParsingCommandLineArguments {
        errors: Box<NonEmpty<IoError<'valid>>>
    }
}

//> STDIOERROR -> INTO ISSUE
impl<'valid> Into<Issue> for IoError<'valid> {
    fn into(self) -> Issue {return match self {
        IoError::OpeningFile {error, name} => Issue {
            name: "failed to open file",
            sections: Vec::from([
                Section::Help(format!(
                    "you might want to create it first{}",
                    match name.as_os_str().to_str() {
                        None => String::default(),
                        Some(string) => format!(": `touch {string}`")
                    }
                )),
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::ReadingMetadata {error} => Issue {
            name: "failed to read file metadata",
            sections: Vec::from([
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::ReadingFile {error} => Issue {
            name: "failed to read file",
            sections: Vec::from([
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::WritingToFile {error} => Issue {
            name: "failed to write to file",
            sections: Vec::from([
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::EncodingUnicode {error} => Issue {
            name: "failed to encode file to UTF-8",
            sections: Vec::from([
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::ParsingSetting {value, numbererror, floaterror} => Issue {
            name: "failed to parse setting value",
            sections: Vec::from([
                Section::Description(format!("failed to parse value {value:?}")),
                Section::Traceback(numbererror.to_string()),
                Section::Traceback(floaterror.to_string())
            ]),
            ..
        },
        IoError::ParsingArgument {argument} => Issue {
            name: "failed to parse argument for command line",
            sections: Vec::from([
                Section::Description(format!("failed to parse argument {argument:?}"))
            ]),
            ..
        },
        IoError::DeterminingPathExists {path, error} => Issue {
            name: "failed to check if path exists",
            sections: Vec::from([
                Section::Description(format!(
                    "couldn't verify {} exists", 
                    path.to_str().map(|name| {
                        format!("{name:?}")
                    }).unwrap_or(String::from("file"))
                )),
                Section::Traceback(error.to_string())
            ]),
            ..
        },
        IoError::ParsingCommandLineArguments {errors} => Issue {
            name: "failed to parse CLI arguments",
            sections: {
                let mut sections = Vec::from([Section::Description(format!(
                    "failed to parse {} argument{}",
                    errors.len(),
                    if errors.len() != 1 {"s"} else {""}
                ))]);
                sections.extend(errors.into_iter().map(|error| {
                    Section::Child(error.into())
                }));
                sections
            },
            ..
        }
    }}
}
