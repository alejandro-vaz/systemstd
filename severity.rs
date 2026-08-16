//^
//^ SEVERITY
//^

//> SEVERITY -> TRAIT
pub trait Severity {
    type Then;
    const COLOR: &'static str;
    const SYMBOL: char;
    fn done() -> Self::Then;
}