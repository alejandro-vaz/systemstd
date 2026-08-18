//^
//^ HANDLING
//^

//> HANDLING -> ENUM
#[derive(Debug)]
pub enum Handling {
    AssumeExists,
    CreateIfMissing,
    AssumeMissing
}