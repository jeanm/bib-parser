use pom::Parser;
use pom::parser::*;
use parser::sp0;
use biblatex::Range;

fn range() -> Parser<u8, Range> {
    let token = || sp0() * none_of(b",-{}\n ")
        .repeat(1..)
        .convert(|bs| String::from_utf8(bs)) - sp0();
    let start = token() - sp0();
    let end = (sym(b'-').repeat(1..) * sp0() * token()).opt();
    let range = start - sp0() + end;
    range.map(|(s, e)| Range { start: s, end: e } )
}

pub fn ranges() -> Parser<u8, Vec<Range>> {
    sym(b'{') * sp0() * list(range(), sp0() * sym(b',') - sp0()) - sp0() - sym(b'}')
}

#[cfg(test)]
mod test {
    use super::*;
    use pom::DataInput;

    #[test]
    fn single() {
        let mut data1 = DataInput::new(b"(a)-- 117");
        let expected1 = Range {
            start: "(a)".to_string(),
            end: Some("117".to_string()),
        };
        assert_eq!(range().parse(&mut data1), Ok(expected1));

        let mut data2 = DataInput::new(b"7");
        let expected2 = Range {
            start: "7".to_string(),
            end: None,
        };
        assert_eq!(range().parse(&mut data2), Ok(expected2));
    }

    #[test]
    fn multiple() {
        let expected1 = Range {
            start: "1".to_string(),
            end: Some("7".to_string()),
        };
        let expected2 = Range {
            start: "10".to_string(),
            end: Some("14".to_string()),
        };
        let expected = vec![expected1, expected2];

        let mut data1 = DataInput::new(b"{1-7, 10--14 }");
        assert_eq!(ranges().parse(&mut data1), Ok(expected.clone()));

        let mut data2 = DataInput::new(b"{ 1 -7 , 10-- 14 }");
        assert_eq!(ranges().parse(&mut data2), Ok(expected));
    }
}