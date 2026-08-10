//^
//^ HEAD
//^

//> HEAD -> STD
use std::fs::OpenOptions;


//^
//^ OPENMODE
//^

//> OPENMODE -> TRAIT
pub trait OpenMode {
    fn setup(options: &mut OpenOptions) -> ();
}

//> OPENMODE -> MARKER WRITEABLE
pub trait Writeable: OpenMode {}

//> OPENMODE -> READ
pub enum Read {} impl OpenMode for Read {
    fn setup(options: &mut OpenOptions) -> () {options.read(true);}
}

//> OPENMODE -> OVERWRITE
pub enum Overwrite {} impl Writeable for Overwrite {} impl OpenMode for Overwrite {
    fn setup(options: &mut OpenOptions) -> () {options.write(true).truncate(true);}
}

//> OPENMODE -> APPEND
pub enum Append {} impl Writeable for Append {} impl OpenMode for Append {
    fn setup(options: &mut OpenOptions) -> () {options.write(true);}
}