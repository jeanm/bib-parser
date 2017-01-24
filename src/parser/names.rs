use pom::Parser;
use pom::parser::*;
use parser::{literal, sp0, sp1, msp0, msp1};
use biblatex::{Name, NameList};

// utility function for the `name()` parser; takes the list of tokens before
// the comma, and optionally the list of tokens after the comma, and identifies
// the given name and the family name
fn split_name(mut before: Vec<String>, after: Option<Vec<String>>) -> Name {
    match after {
        None => match before.len() {
            0 => unreachable!(),
            // no commas, only one token
            1 => {
                Name {
                    family: before.pop().unwrap(),
                    .. Name::default()
                }
            }
            // no commas, multiple tokens: last is family
            _ => {
                Name {
                    family: before.pop().unwrap(),   
                    given: Some(before.join(" ")),
                }
            }
        },
        Some(after) => {
            Name {
                family: before.join(" "),
                given: Some(after.join(" ")),
            }
        }
    }
}

// space-or-comma-delimited name token
fn name_token() -> Parser<u8, String> {
    let simple = none_of(b" ,{}\t\n\r\\").repeat(1..).convert(|bs| String::from_utf8(bs));
    let sequence = (simple | literal()).repeat(1..);
    sequence.map(|ss| ss.concat())
}

// comma-delimited name part
fn name_part() -> Parser<u8, Vec<String>> {
    // TODO fix this ugly lookahead hack
    list(name_token(), sp1() - !((seq(b"and") - msp1())| seq(b"}")))
}

// single name
fn name() -> Parser<u8, Name> {
    let name = name_part() + (sp0() * sym(b',') * sp0() * name_part()).opt();
    name.map(|(b, a)| split_name(b, a))
}

/// List of names, used in fields such as `author` and `editor`.
pub fn name_list() -> Parser<u8, NameList> {
    let names = list(name(), msp1() * seq(b"and") - msp1());
    let delimited = sym(b'{') * msp0() * names - msp0() - sym(b'}');
    delimited.map(|ns| NameList::from_names(ns))
}

#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;

    #[test]
    fn single() {
        let mut data = DataInput::new(b"{'t Hooft}, Gerard and  junk");
        let expected = Name {
            family: "'t Hooft".to_string(),
            given: Some("Gerard".to_string()),
        };
        assert_eq!(name().parse(&mut data), Ok(expected));

        let mut data = DataInput::new(b"Gerard {'t Hooft}");
        let expected = Name {
            family: "'t Hooft".to_string(),
            given: Some("Gerard".to_string()),
        };
        assert_eq!(name().parse(&mut data), Ok(expected));
    }

    #[test]
    fn list() {
        let name1 = Name {
            family: "'t Hooft".to_string(),
            given: Some("Gerard".to_string()),
        };
        let name2 = Name {
            family: "Celentano".to_string(),
            given: Some("A. Driano".to_string()),
        };
        let expected = NameList {
            names: vec![name1, name2],
            truncated: false,
        };

        // comma-separated
        let mut data = DataInput::new(b"{'t Hooft, Gerard and Celentano, A. Driano}");
        assert_eq!(name_list().parse(&mut data), Ok(expected.clone()));

        // mixed
        let mut data = DataInput::new(b"{'t Hooft, Gerard and A. Driano Celentano}");
        assert_eq!(name_list().parse(&mut data), Ok(expected.clone()));

        // no commas
        let mut data = DataInput::new(b"{Gerard {'t Hooft} and A. Driano Celentano}");
        assert_eq!(name_list().parse(&mut data), Ok(expected.clone()));

        // newline
        let mut data = DataInput::new(b"{Gerard {'t Hooft} and\n Celentano, A. Driano }");
        assert_eq!(name_list().parse(&mut data), Ok(expected.clone()));

        // weird spacing
        let mut data = DataInput::new(b"{ 't Hooft,Gerard \n  and  Celentano  , A.  Driano }");
        assert_eq!(name_list().parse(&mut data), Ok(expected));
    }
}
