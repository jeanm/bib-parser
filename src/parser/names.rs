use pom::Parser;
use pom::parser::*;
use parser::{literal, sp0, sp1};
use biblatex::{Name, NameList};

// space-or-comma-delimited name token
fn name_token() -> Parser<u8, String> {
    let simple = none_of(b" ,{}\t\n\r\\").repeat(1..).convert(|bs| String::from_utf8(bs));
    let sequence = (simple | literal()).repeat(1..);
    sequence.map(|ss| ss.concat())
}

// comma-delimited name part
fn name_part() -> Parser<u8, String> {
    // TODO fix this ugly lookahead hack
    let tokens = list(name_token(), sp1() - !(seq(b"and ") | seq(b"}")));
    tokens.map(|ss| ss.join(" "))
}

// single name
fn name() -> Parser<u8, Name> {
    let name = name_part() + (sp0() * sym(b',') * sp0() * name_part()).opt();
    name.map(|(f, g)| Name { family: f, given: g })
}

/// List of names, used in fields such as `author` and `editor`.
pub fn name_list() -> Parser<u8, NameList> {
    let names = list(name(), sp1() * seq(b"and") - sp1());
    let delimited = sym(b'{') * sp0() * names - sp0() - sym(b'}');
    delimited.map(|ns| NameList::from_names(ns))
}

#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;

    #[test]
    fn part() {
        let mut data = DataInput::new(b"'t  Hooft,junkjunkjunk");
        let expected = "'t Hooft".to_string();
        assert_eq!(name_part().parse(&mut data), Ok(expected));
    }

    #[test]
    fn single() {
        let mut data = DataInput::new(b"{'t Hooft}, Gerard and  junk");
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

        let mut data1 = DataInput::new(b"{'t Hooft, Gerard and Celentano, A. Driano}");
        assert_eq!(name_list().parse(&mut data1), Ok(expected.clone()));

        let mut data2 = DataInput::new(b"{'t Hooft, Gerard and Celentano, A. Driano }");
        assert_eq!(name_list().parse(&mut data2), Ok(expected));
    }
}
