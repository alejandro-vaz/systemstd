//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    fs::{
        File as StdFile, 
        exists
    }, 
    path::PathBuf
};

//> HEAD -> SUPER
use super::{
    descriptor::Descriptor,
    ioerror::IoError,
    openmode::OpenMode,
    handling::Handling
};


//^ 
//^ PATH
//^

//> PATH -> STRUCT
pub struct Path {
    name: PathBuf
}

//> PATH -> IMPLEMENTATION
impl Path {
    pub fn exists<'valid>(&'valid self) -> Result<bool, IoError<'valid>> {
        return exists(&self.name).map_err(|error| IoError::CantKnowIfPathExists {
            path: &self.name, 
            error: error 
        })
    }
    pub fn file<Mode: OpenMode>(
        self, 
        handling: Handling
    ) -> Result<Descriptor<Mode>, IoError<'static>> {
        let mut options = StdFile::options();
        Mode::setup(&mut options);
        match handling {
            Handling::AssumeExists => options.create(false).create_new(false),
            Handling::CreateIfMissing => options.create(true),
            Handling::AssumeMissing => options.create_new(true)
        };
        return match options.open(&self.name) {
            Ok(stdfile) => Ok(Descriptor::from(stdfile)),
            Err(error) => Err(IoError::CouldntOpenFile {
                error: error, 
                name: self.name
            })
        }
    }
}

//> PATH -> FROM PATHBUF
impl From<PathBuf> for Path {
    fn from(value: PathBuf) -> Self {return Self {
        name: value
    }}
}