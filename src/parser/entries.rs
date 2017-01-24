use std::error::Error;
use std::fmt;
use pom::Parser;
use pom::parser::*;
use pom::char_class::alphanum;
use parser::{sp0, msp0, field};
use biblatex::{Entry, Field};

// recognises chars that can make up a citation key: a-zA-Z0-9_/-
fn cite_key_char(c: u8) -> bool {
    alphanum(c) || c == b'_' || c == b'/' || c == b'-'
}

#[derive(Debug)]
struct MatchFail;

impl fmt::Display for MatchFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String match failed.")
    }
}

impl Error for MatchFail {
    fn description(&self) -> &str { "String match failed." }
    fn cause(&self) -> Option<&Error> { None }
}

// attempts to match some bytes in a case-insensitive way,
// errors out if it fails (so that the parser can backtrack)
fn insensitive_match(bytes: Vec<u8>, rhs: &'static str) -> Result<(), Box<Error>> {
    let lowercase = String::from_utf8(bytes)?.to_lowercase();
    if lowercase == rhs {
        Ok(())
    } else {
        Err(Box::new(MatchFail))
    }
}

// bibliographic entry tag, e.g. `@article`
fn tag(name: &'static str) -> Parser<u8, ()> {
    let matcher = move |bs| insensitive_match(bs, name);
    sym(b'@') * is_a(alphanum).repeat(1..).convert(matcher).discard()
}

fn cite_key() -> Parser<u8, String> {
    is_a(cite_key_char).repeat(1..).convert(|bs| String::from_utf8(bs))
}

fn entry_template(name: &'static str) -> Parser<u8, (String, Vec<Field>)> {
    let tag_line = tag(name) * msp0() * sym(b'{') * msp0();
    let open = tag_line * cite_key() - sp0() - sym(b',') - msp0();
    let close = sym(b',').opt() * msp0() * sym(b'}');
    let fields = list(field(), sp0() * sym(b',') - msp0() -!sym(b'}'));
    open + fields - close
}

pub fn entry() -> Parser<u8, (String, Option<Entry>)> {
    entry_template("article").map(|(k, fs)| (k, Entry::new_article(fs))) |
    entry_template("inproceedings").map(|(k, fs)| (k, Entry::new_in_proceedings(fs)))
}

#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;
    use biblatex::{Article, Name, NameList};

    #[test]
    fn match_tag() {
        let mut data = DataInput::new(b"@ThiSiSAtaG");
        assert_eq!(tag("thisisatag").parse(&mut data), Ok(()));
    }

    #[test]
    fn article() {
        let raw = br#"@article{baez/article,
  author       = {Baez, John C. and Lauda, Aaron D.},
  title        = {Higher-Dimensional Algebra {V}: 2-Groups},
  journaltitle = {Theory and Applications of Categories},
  date         = 2004,
  volume       = 12,
}"#;
        let mut data = DataInput::new(raw);
        let name1 = Name {
            family: "Baez".to_string(),
            given: Some("John C.".to_string()),
        };
        let name2 = Name {
            family: "Lauda".to_string(),
            given: Some("Aaron D.".to_string()),
        };
        let author = NameList {
            names: vec![name1, name2],
            truncated: false,
        };
        let expected = (
            "baez/article".to_string(),
            Some(Entry::Article(
                Article {
                    author: author,
                    title: "Higher-Dimensional Algebra V: 2-Groups".to_string(),
                    journal_title: "Theory and Applications of Categories".to_string(),
                    year: 2004,
                    volume: Some("12".to_string()),
                    .. Article::default()
                }
            ))
        );
        assert_eq!(entry().parse(&mut data), Ok(expected));
    }
}