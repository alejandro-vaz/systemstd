//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

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


//^
//^ STDIOERROR
//^

//> STDIOERROR -> ENUM
#[derive(Debug)]
pub enum IoError<'valid> {
    CouldntOpenFile {
        error: Error,
        name: PathBuf
    },
    CouldntReadMetadata {
        error: Error
    },
    CouldntReadFile {
        error: Error
    },
    CouldntWriteToFile {
        error: Error
    },
    FailedToEncodeRead {
        error: FromUtf8Error
    },
    UnknownSettingValue {
        value: String,
        errors: (ParseIntError, ParseFloatError)
    },
    FailureParsingArgument {
        argument: String
    },
    CantKnowIfPathExists {
        path: &'valid PathBuf,
        error: Error
    }
}

//> STDIOERROR -> INTO ISSUE
impl<'valid> Into<Issue> for IoError<'valid> {
    fn into(self) -> Issue {return match self {
        IoError::CouldntOpenFile {error, name} => Issue {
            name: "failed to open file",
            help: try {format!(
                "you might want to create it first: `touch {}`", 
                name.as_os_str().to_str()?
            )},
            traceback: Some(error.to_string()),
            ..
        },
        IoError::CouldntReadMetadata {error} => Issue {
            name: "failed to read file metadata",
            traceback: Some(error.to_string()),
            ..
        },
        IoError::CouldntReadFile {error} => Issue {
            name: "failed to read file",
            traceback: Some(error.to_string()),
            ..
        },
        IoError::CouldntWriteToFile {error} => Issue {
            name: "failed to write to file",
            traceback: Some(error.to_string()),
            ..
        },
        IoError::FailedToEncodeRead {error} => Issue {
            name: "failed to encode file to UTF-8",
            traceback: Some(error.to_string()),
            ..
        },
        IoError::UnknownSettingValue {value, ..} => Issue {
            name: "failed to parse setting value",
            description: Some(format!("string to parse: {value:?}")),
            ..
        },
        IoError::FailureParsingArgument {argument} => Issue {
            name: "failed to parse argument for command line",
            description: Some(format!("string to parse: {argument:?}")),
            ..
        },
        IoError::CantKnowIfPathExists {path, error} => Issue {
            name: "failed to check if path exists",
            description: try {format!("couldn't verify {:?} exists", path.to_str()?)},
            traceback: Some(error.to_string()),
            ..
        }
    }}
}
