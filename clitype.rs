//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::ioerror::IoError;


//^
//^ CLITYPE
//^

//> CLITYPE -> ENUM
#[derive(Debug, Clone)]
pub enum CliType {
    Integer {
        value: u128
    },
    Float {
        value: f64
    }
}

//> CLITYPE -> TRY FROM STR
impl TryFrom<String> for CliType {
    type Error = IoError<'static>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let number = match value.parse::<u128>() {
            Ok(value) => return Ok(CliType::Integer {
                value: value
            }),
            Err(error) => error
        };
        let float = match value.parse::<f64>() {
            Ok(value) => return Ok(CliType::Float {
                value: value
            }),
            Err(error) => error
        };
        return Err(IoError::UnknownSettingValue { 
            value: value, 
            errors: (number, float)
        });
    }
}