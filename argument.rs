//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    clitype::CliType,
    ioerror::IoError
};

//> HEAD -> STD
use std::path::PathBuf;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ ARGUMENT
//^

//> ARGUMENT -> ENUM
#[derive(Debug, Clone, EnumAsInner)]
pub enum Argument {
    Path {
        buffer: PathBuf
    },
    Alias {
        characters: Vec<char>
    },
    Flag {
        value: String
    },
    Setting {
        key: String, 
        value: CliType
    },
    Target {
        to: String
    }
} 

//> ARGUMENT -> PARSING
impl TryFrom<String> for Argument {
    type Error = IoError<'static>;
    fn try_from(value: String) -> Result<Self, Self::Error> {return Ok(match value {
        ref capture if let Some(Some((key, value))) = capture.strip_prefix("--").map(|item| {
            item.split_once('=')
        }) => Argument::Setting {
            key: key.to_string(), 
            value: CliType::try_from(value.to_string())?
        },
        ref capture if let Some(flag) = capture.strip_prefix("--") => Argument::Flag {
            value: flag.to_string()
        },
        ref capture if let Some(alias) = capture.strip_prefix('-') => Argument::Alias {
            characters: alias.chars().collect()
        },
        other => {
            let mut path = false;
            for character in other.chars() {match character {
                num if num.is_numeric() => path = true,
                '/' | '\\' | '.' | ':' | '_' => path = true,
                letter if letter.is_alphabetic() => (),
                _ => return Err(IoError::FailureParsingArgument {
                    argument: other
                })
            }};
            if path {Argument::Path {
                buffer: PathBuf::from(other)
            }} else {Argument::Target {
                to: other
            }}
        }
    })}
}