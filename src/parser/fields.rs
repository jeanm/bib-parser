use std::str::FromStr;
use pom::Parser;
use pom::parser::*;
use pom::char_class::digit;
use parser::{literal, sp0, ranges, name_list};
use biblatex::Field;

pub fn field() -> Parser<u8, Field> {
    title_field() |
    book_title_field() |
    journal_title_field() |
    year_or_date_field() |
    pages_field() |
    author_field() |
    editor_field() |
    url_field() |
    volume_field() |
    series_field() |
    issue_field() |
    unknown_field()
}

fn title_field() -> Parser<u8, Field> {
    let title = call(literal).map(|s| Field::Title(s));
    seq(b"title") * sp0() * sym(b'=') * sp0() * title
}

fn book_title_field() -> Parser<u8, Field> {
    let title = call(literal).map(|s| Field::BookTitle(s));
    seq(b"booktitle") * sp0() * sym(b'=') * sp0() * title
}

fn journal_title_field() -> Parser<u8, Field> {
    let title = call(literal).map(|s| Field::JournalTitle(s));
    seq(b"journaltitle") * sp0() * sym(b'=') * sp0() * title
}

fn year_or_date_field() -> Parser<u8, Field> {
    // we extract the year from the `year` field
    let four_digits = || is_a(digit).repeat(4..5).convert(|bs| String::from_utf8(bs));
    let year = || four_digits().convert(|s| i32::from_str(&s)).map(|i| Field::Year(i));
    let year_braced = sym(b'{') * sp0() * year() - sp0() - sym(b'}');
    let year_field = seq(b"year") * sp0() * sym(b'=') * sp0() * (year() | year_braced);

    // we extract *just* the year from the `date` field
    let ext_digits = sym(b'{') * sp0() * four_digits() - none_of(b"}\n").repeat(0..) - sym(b'}');
    let date = ext_digits.convert(|s| i32::from_str(&s)).map(|i| Field::Year(i)) | year();
    let date_field = seq(b"date") * sp0() * sym(b'=') * sp0() * date;

    year_field | date_field
}

fn pages_field() -> Parser<u8, Field> {
    let pages = ranges().map(|rs| Field::Pages(rs));
    seq(b"pages") * sp0() * sym(b'=') * sp0() * pages
}

fn author_field() -> Parser<u8, Field> {
    let authors = name_list().map(|ns| Field::Author(ns));
    seq(b"author") * sp0() * sym(b'=') * sp0() * authors
}

fn editor_field() -> Parser<u8, Field> {
    let editors = name_list().map(|ns| Field::Editor(ns));
    seq(b"editor") * sp0() * sym(b'=') * sp0() * editors
}

fn url_field() -> Parser<u8, Field> {
    let url = call(literal).map(|s| Field::Url(s));
    seq(b"url") * sp0() * sym(b'=') * sp0() * url
}

fn volume_field() -> Parser<u8, Field> {
    let volume_literal = call(literal);
    let volume_numerical = is_a(digit).repeat(1..).convert(|bs| String::from_utf8(bs));
    let volume = (volume_literal | volume_numerical).map(|s| Field::Volume(s));
    seq(b"volume") * sp0() * sym(b'=') * sp0() * volume
}

fn series_field() -> Parser<u8, Field> {
    let series_literal = call(literal);
    let series_numerical = is_a(digit).repeat(1..).convert(|bs| String::from_utf8(bs));
    let series = (series_literal | series_numerical).map(|s| Field::Series(s));
    seq(b"series") * sp0() * sym(b'=') * sp0() * series
}

fn issue_field() -> Parser<u8, Field> {
    let issue = call(literal).map(|s| Field::Issue(s));
    seq(b"issue") * sp0() * sym(b'=') * sp0() * issue
}

fn unknown_field() -> Parser<u8, Field> {
    let name = none_of(b"{}=\n").repeat(1..).discard();
    let some_string = none_of(b",\n{}").repeat(0..).discard();
    let unknown = name * sp0() * sym(b'=') * sp0() * (literal().discard() | some_string);
    unknown.collect().convert(|bs| String::from_utf8(bs)).map(|s| Field::Unknown(s))
}


#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;
    use biblatex::{Name, NameList, Range};

    #[test]
    fn titles() {
        let mut data1 = DataInput::new(b"title={This is a title}");
        let expected1 = Field::Title("This is a title".to_string());
        assert_eq!(field().parse(&mut data1), Ok(expected1));

        let mut data2 = DataInput::new(b"booktitle={This is a {Title}}");
        let expected2 = Field::BookTitle("This is a Title".to_string());
        assert_eq!(field().parse(&mut data2), Ok(expected2));
    }

    #[test]
    fn series() {
        let mut data = DataInput::new(b"series  = 1");
        let expected = Field::Series("1".to_string());
        assert_eq!(field().parse(&mut data), Ok(expected));

        let mut data = DataInput::new(b"series   = {Moreshet: Studies in {Jewish} History, Literature and Thought}");
        let expected = Field::Series("Moreshet: Studies in Jewish History, Literature and Thought".to_string());
        assert_eq!(field().parse(&mut data), Ok(expected));
    }

    #[test]
    fn volume() {
        let mut data = DataInput::new(b"volume    = 1");
        let expected = Field::Volume("1".to_string());
        assert_eq!(field().parse(&mut data), Ok(expected));

        let mut data = DataInput::new(b"volume={A}");
        let expected = Field::Volume("A".to_string());
        assert_eq!(field().parse(&mut data), Ok(expected));
    }

    #[test]
    fn year() {
        let mut data = DataInput::new(b"year=  2017");
        let expected = Field::Year(2017);
        assert_eq!(field().parse(&mut data), Ok(expected));

        let mut data = DataInput::new(b"year = { 2017}");
        let expected = Field::Year(2017);
        assert_eq!(field().parse(&mut data), Ok(expected));

        let mut data = DataInput::new(b"date   = {2000-12-01}");
        let expected = Field::Year(2000);
        assert_eq!(field().parse(&mut data), Ok(expected));
    }

    #[test]
    fn pages() {
        let mut data = DataInput::new(b"pages  =  {100-102}");
        let range = Range {
            start: "100".to_string(),
            end: Some("102".to_string()),
        };
        let expected = Field::Pages(vec![range]);
        assert_eq!(field().parse(&mut data), Ok(expected));
    }

    #[test]
    fn author() {
        let mut data = DataInput::new(b"author       = {Baez, John C. and Lauda, Aaron D.}");
        let name1 = Name {
            family: "Baez".to_string(),
            given: Some("John C.".to_string()),
        };
        let name2 = Name {
            family: "Lauda".to_string(),
            given: Some("Aaron D.".to_string()),
        };
        let expected = Field::Author(NameList {
            names: vec![name1, name2],
            truncated: false,
        });
        assert_eq!(field().parse(&mut data), Ok(expected));
    }
}