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
    "[bold {}]{} {}[/][bold]: {}[/]{}",
    Mode::COLOR,
    Mode::SYMBOL,
    {
        let name = type_name::<Mode>().as_bytes();
        let mut first = 0;
        for index in 0.. {
            let byte = name.get(index);
            match byte {
                None | Some(b'<') => break,
                Some(b':') => first = index + 1,
                Some(_) => continue
            }
        }
        unsafe {str::from_utf8_unchecked(&name[first..])}
    },
    issue.name,
    {
        let mut sections = issue.sections.into_iter().map(|section| {match section {
            Section::Child(issue) => format!("\n{}", display::<Mode>(issue)),
            Section::Code {
                code, 
                message: _message, 
                line, 
                span: _span, 
                language: _language
            } => format!(
                "\n[on black]{}    {code}[/]\n",
                if let Some(number) = line {
                    format!("[gray]{number:>4}[/]")
                } else {String::new()}
            ),
            Section::Description(string) => string,
            Section::Help(string) => format!("[cyan]help[/]: {string}"),
            Section::Note(string) => format!("[yellow]note[/]: {string}"),
            Section::Traceback(string) => format!("[gray]traceback[/]: {string}")
        }}).collect::<Vec<String>>().join("\n");
        if !sections.is_empty() {
            sections.insert_str(0, "\n")
        };
        let mut new = sections.replace('\n', "\n  [gray]|[/] ");
        if !sections.is_empty() {new.push('\n');}
        new
    }
)}