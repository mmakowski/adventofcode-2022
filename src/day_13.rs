use std::cmp::Ordering;
use std::str::FromStr;
use Elem::*;

use Error::*;

#[derive(Debug)]
pub enum Error {
    ExtraInput(String),
    ParseElem(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Elem {
    Int(u8),
    List(Vec<Elem>),
}

impl FromStr for Elem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (extra_input, elem) = parse::elem(s.as_bytes())
            .map_err(|e| {
                let s = match e {
                    nom::Err::Error(e) => format!("error: {}", String::from_utf8_lossy(e.input)),
                    nom::Err::Incomplete(n) => format!("needed: {n:?}"),
                    nom::Err::Failure(e) => format!("failure: {}", String::from_utf8_lossy(e.input)),
                };
                ParseElem(s.to_string())
            })?;
        if !extra_input.is_empty() {
            Err(ExtraInput(String::from_utf8_lossy(extra_input).to_string()))
        } else {
            Ok(elem)
        }
    }
}

impl Eq for Elem {}

impl PartialOrd<Self> for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(x), Int(y)) => x.cmp(y),
            (List(x), List(y)) => {
                match (x.is_empty(), y.is_empty()) {
                    (true, true) => Ordering::Equal,
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => {
                        let e1c = x[0].cmp(&y[0]);
                        if e1c == Ordering::Equal {
                            List(x[1..].to_vec()).cmp(&List(y[1..].to_vec()))
                        } else {
                            e1c
                        }
                    }
                }
            },
            (Int(_), l) => List(vec![self.clone()]).cmp(l),
            (l, Int(_)) => l.cmp(&List(vec![other.clone()])),
        }
    }
}

mod parse {
    use std::str;

    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::is_digit;
    use nom::Err::{Error, Failure};
    use nom::error::ErrorKind::Digit;
    use nom::IResult;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;

    use super::Elem;
    use super::Elem::*;

    pub fn elem(input: &[u8]) -> IResult<&[u8], Elem> {
        alt((int, list))(input)
    }

    pub fn list(input: &[u8]) -> IResult<&[u8], Elem> {
        let (rest, elems) = delimited(tag("["), separated_list0(tag(","), elem), tag("]"))(input)?;
        Ok((rest, List(elems)))
    }

    pub fn int(input: &[u8]) -> IResult<&[u8], Elem> {
        let (rest, digits) = take_while(is_digit)(input)?;
        // TODO: more elegant error handling
        if digits.is_empty() {
            Err(Error(nom::error::Error::new(input, Digit)))
        } else {
            let s = str::from_utf8(digits).map_err(|_| Failure(nom::error::Error::new(input, Digit)))?;
            let num = s.parse::<u8>().map_err(|_| Failure(nom::error::Error::new(input, Digit)))?;
            Ok((rest, Int(num)))
        }
    }
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn parse() {
        let num = "123".parse::<Elem>().unwrap();
        assert_eq!(num, Int(123));
        let lst = "[1,2,[3],4,[]]".parse::<Elem>().unwrap();
        assert_eq!(lst, List(vec![Int(1), Int(2), List(vec![Int(3)]), Int(4), List(vec![])]));
    }

    #[test]
    fn ord() {
        assert!(Int(0) < Int(1));
        assert!(List(vec![]) < List(vec![Int(0)]));
        assert!(List(vec![]) < Int(0));
        assert!(List(vec![Int(0)]) < Int(1));
    }
}
/*
grammar:
<packet> ::= <list>
<list> ::= [ <content> ]
<content> ::= <elem> | <elem> , <content>
<elem> ::= <num> | <list>
 */