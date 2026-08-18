//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;


//^
//^ SEVERITY
//^

//> SEVERITY -> TRAIT
pub const trait Severity: Into<Issue> {
    type Then;
    fn done() -> Self::Then;
}

//> SEVERITY -> ISSUE IMPLEMENTATION
impl Severity for Issue {
    type Then = ();
    fn done() -> Self::Then {}
}