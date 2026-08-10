//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> CORE
use core::fmt::{
    Result as Format,
    Formatter,
    Display
};


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem {
    pub issue: Issue,
    pub severity: Option<bool>
}

//> PROBLEM -> DISPLAY
impl Display for Problem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "{}[bold]: {}[/]{}",
        match self.severity {
            None => "[bold red]Critical[/]",
            Some(false) => "[bold red]Warning[/]",
            Some(true) => "[bold yellow]Error[/]"
        },
        self.issue.name,
        format!(
            "{}{}{}",
            self.issue.traceback.as_ref().map(|string| {
                format!("\n[gray]traceback[/]: {string}")
            }).unwrap_or_default(),
            self.issue.description.as_ref().map(|string| {
                format!("\n{string}")
            }).unwrap_or_default(),
            self.issue.help.as_ref().map(|string| {
                format!("\n[blue]help[/]: {string}")
            }).unwrap_or_default()
        ).replace('\n', "\n    ")
    )}
}