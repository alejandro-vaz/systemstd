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
    error::Error,
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
    pub fn metadata(&self) -> Result<Metadata, Error<'static>> {
        return match self.stdfile.metadata() {
            Ok(metadata) => Ok(Metadata::from(metadata)),
            Err(ioerror) => Err(Error::ReadingMetadata {
                ioerror: ioerror
            })
        }
    }
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, Error<'static>> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.stdfile.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(ioerror) => Err(Error::ReadingFile {
                ioerror: ioerror
            })
        }
    }
    pub fn read(&mut self) -> Result<String, Error<'static>> {
        return String::from_utf8(self.read_bytes()?).map_err(|ioerror| {
            Error::EncodingUnicode {
                ioerror: ioerror
            }
        });
    }
}

//> DESCRIPTOR -> WRITE IMPLEMENTATION
impl<Mode: Writeable> Descriptor<Mode> {
    pub fn write_bytes(&mut self, content: &[u8]) -> Result<(), Error<'static>> {
        return match self.stdfile.write(content) {
            Ok(_) => Ok(()),
            Err(ioerror) => Err(Error::WritingToFile {
                ioerror: ioerror
            })
        }
    }
    pub fn write(&mut self, content: &str) -> Result<(), Error<'static>> {
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