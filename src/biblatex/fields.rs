use std::fmt;

/// Field specifying some bibliographic information
#[derive(Clone, PartialEq, Debug)]
pub enum Field {
    Author(NameList),
    Editor(NameList),
    Title(String),
    JournalTitle(String),
    BookTitle(String),
    Year(i32),
    Pages(Vec<Range>),
    Url(String),
    Volume(String),
    Series(String),
    Issue(String),
    Unknown(String),
}

/// Range type, typically used by the `pages` field
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Range {
    pub start: String,
    pub end: Option<String>
}

/// Name of a person or organisation
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Name {
    pub family: String,
    pub given: Option<String>,
    // TODO: deal with prefixes/suffixes
}

/// Name list type, typically used by the `author` or `editor` field
///
/// This can be optionally truncated, in BibLaTeX normally by writing `and others`.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct NameList {
    pub names: Vec<Name>,
    pub truncated: bool,
}

impl NameList {
    pub fn from_names(names: Vec<Name>) -> NameList {
        // TODO: check if last name is "others", set `truncated`
        NameList {
            names: names,
            truncated: false,
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.family)
    }
}

impl fmt::Display for NameList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut names: Vec<String> = Vec::new();
        for name in &self.names {
            names.push(name.to_string());
        }
        write!(f, "{}", names.join(", "))
    }
}