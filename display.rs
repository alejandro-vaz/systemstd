//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> SUPER
use super::severity::Severity;

//> HEAD -> CORE
use core::any::type_name;


//^
//^ DISPLAY
//^

//> DISPLAY -> FUNCTION
pub fn display<Mode: Severity>(issue: Issue) -> String {return format!(
    "[error]# {}[/][bold]: {}[/]{}",
    name::<Mode>(),
    issue.name,
    {
        let mut sections = issue.sections.into_iter().map(|next| {
            section::<Mode>(next)
        }).collect::<Vec<String>>();
        if !sections.is_empty() {
            sections.insert(0, String::from(""))
        };
        sections.join("\n").replace('\n', "\n  [basic]|[/] ")
    }
)}

//> DISPLAY -> SECTION
fn section<Mode: Severity>(section: Section) -> String {return match section {
    Section::Child(issue) => format!("{}", display::<Mode>(issue)),
    Section::Code {
        extends,
        code, 
        language: _language,
        path: _path,
        line,
        span: _span
    } => format!(
        "> {}\n>\n> [basic]{}[/]  {code}\n>",
        self::section::<Mode>(*extends),
        if let Some(number) = line {
            format!("{number:>4}")
        } else {String::from("    ")}
    ),
    Section::Help(string) => format!("[help]help[/]: {string}"),
    Section::Note(string) => format!("[note]note[/]: {string}"),
    Section::Cause(string) => format!("[cause]cause[/]: {string}"),
    Section::Deprecated(string) => format!("[deprecated]deprecated[/]: {string}")
}}

//> DISPLAY -> NAME
fn name<Mode: Severity>() -> &'static str {
    let name = type_name::<Mode>().as_bytes();
    let mut first = 0;
    let mut last = None;
    for index in 0.. {
        match name.get(index) {
            None | Some(b'<') => {
                last = Some(index);
                break
            },
            Some(b':') => first = index + 1,
            Some(_) => continue
        }
    }
    return match last {
        None => unsafe {str::from_utf8_unchecked(&name[first..])},
        Some(at) => unsafe {str::from_utf8_unchecked(&name[first..at])}
    };
}