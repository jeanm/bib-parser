use pom::{DataInput, Parser, Error};
use pom::parser::*;
use pom::char_class::{space, multispace};
use parser::ranges::ranges;
use parser::names::name_list;
use parser::fields::field;
use parser::entries::entry;
use biblatex::Entry;

mod fields;
mod ranges;
mod names;
mod entries;

// spacing (zero or more)
fn sp0() -> Parser<u8, ()> {
    is_a(space).repeat(0..).discard()
}

// spacing+newlines (zero or more)
fn msp0() -> Parser<u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

// spacing (one or more)
fn sp1() -> Parser<u8, ()> {
    is_a(space).repeat(1..).discard()
}

// match a braced literal expression, return its contents
fn literal() -> Parser<u8, String> {
    let simple = none_of(b"{}").repeat(1..).convert(|bs| String::from_utf8(bs));
    let nested = call(literal);
    let content = (simple | nested).repeat(0..).map(|ref ss| ss.concat());
    sym(b'{') * content - sym(b'}')
}

pub fn parse_bib(buf: &[u8]) -> Result<Vec<(String, Option<Entry>)>, Error> {
    let parser = (msp0() * entry() - msp0()).repeat(0..);
    parser.parse(&mut DataInput::new(buf))
}


#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;

    #[test]
    fn simple_literal() {
        let mut data = DataInput::new(b"{This is a simple literal}");
        let expected = "This is a simple literal".to_string();
        assert_eq!(literal().parse(&mut data), Ok(expected));
    }

    #[test]
    fn nested_literal() {
        let mut data = DataInput::new(b"{This is a {nested literal}}");
        let expected = "This is a nested literal".to_string();
        assert_eq!(literal().parse(&mut data), Ok(expected));
    }
}