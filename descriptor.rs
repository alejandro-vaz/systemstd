//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    fs::File as StdFile,
    io::Read
};

//> HEAD -> SUPER
use super::{
    metadata::Metadata,
    ioerror::IoError,
    openmode::{
        OpenMode,
        Writeable
    }
};

//> HEAD -> CORE
use core::{
    io::Write,
    marker::PhantomData
};


//^
//^ DESCRIPTOR
//^

//> DESCRIPTOR -> STRUCT
pub struct Descriptor<Mode: OpenMode> {
    stdfile: StdFile,
    mode: PhantomData<Mode>
}

//> DESCRIPTOR -> IMPLEMENTATION
impl<Mode: OpenMode> Descriptor<Mode> {
    pub fn metadata(&self) -> Result<Metadata, IoError<'static>> {
        return match self.stdfile.metadata() {
            Ok(metadata) => Ok(Metadata::from(metadata)),
            Err(error) => Err(IoError::ReadingMetadata {error: error})
        }
    }
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, IoError<'static>> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.stdfile.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(IoError::ReadingFile {error: error})
        }
    }
    pub fn read(&mut self) -> Result<String, IoError<'static>> {
        return String::from_utf8(self.read_bytes()?).map_err(|error| {
            IoError::EncodingUnicode {error: error}
        });
    }
}

//> DESCRIPTOR -> WRITE IMPLEMENTATION
impl<Mode: Writeable> Descriptor<Mode> {
    pub fn write_bytes(&mut self, content: &[u8]) -> Result<(), IoError<'static>> {
        return match self.stdfile.write(content) {
            Ok(_) => Ok(()),
            Err(error) => Err(IoError::WritingToFile {error: error})
        }
    }
    pub fn write(&mut self, content: &str) -> Result<(), IoError<'static>> {
        return self.write_bytes(content.as_bytes())
    }
}

//> DESCRIPTOR -> FROM STDFILE
impl<Mode: OpenMode> From<StdFile> for Descriptor<Mode> {
    fn from(value: StdFile) -> Self {return Self {
        stdfile: value,
        mode: PhantomData
    }}
}