use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::is_digit;
use nom::Err::{Error, Failure};
use nom::error::ErrorKind::Digit;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use std::str;

use Elem::*;

#[derive(Debug, PartialEq, Clone)]
enum Elem {
    Int(u8),
    List(Vec<Elem>),
}

fn list(input: &[u8]) -> IResult<&[u8], Elem> {
    let (rest, elems) = delimited(tag("["), separated_list0(tag(","), list_elem), tag("]"))(input)?;
    Ok((rest, List(elems)))
}

fn list_elem(input: &[u8]) -> IResult<&[u8], Elem> {
    alt((int, list))(input)
}

fn int(input: &[u8]) -> IResult<&[u8], Elem> {
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

#[cfg(test)]
mod run {
    use nom::error::dbg_dmp;
    use super::*;

    #[test]
    fn parse() {
        let (_rest, num) = int(&b"123"[..]).unwrap();
        assert_eq!(num, Int(123));
        let (_rest, lst) = dbg_dmp(list, "TEST")(&b"[1,2,[3],4,[]]"[..]).unwrap();
        assert_eq!(lst, List(vec![Int(1), Int(2), List(vec![Int(3)]), Int(4), List(vec![])]));
    }
}
/*
grammar:
<packet> ::= <list>
<list> ::= [ <content> ]
<content> ::= <elem> | <elem> , <content>
<elem> ::= <num> | <list>
 */