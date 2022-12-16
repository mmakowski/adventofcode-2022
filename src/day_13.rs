use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use Elem::*;

use Error::*;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ExtraInput(String),
    ParseElem(String),
    NotFound(Elem)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Elem {
    Int(u8),
    List(Vec<Elem>),
}

fn count_right_order() -> Result<u64, Error> {
    let mut i = 0;
    let mut result = 0;
    let mut l1 = Int(0);
    let file = File::open("input-13.txt").map_err(|e| IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        match i % 3 {
            0 => l1 = line.parse::<Elem>()?,
            1 => {
                let l2 = line.parse::<Elem>()?;
                if l1 < l2 {
                    result += (i + 2) / 3 // add 1-based index of the pair
                }
            }
            _ => ()
        }
        i += 1;
    }
    Ok(result)
}

fn decoder_key() -> Result<usize, Error> {
    let file = File::open("input-13.txt").map_err(|e| IO(e))?;
    let p1 = List(vec![List(vec![Int(2)])]);
    let p2 = List(vec![List(vec![Int(6)])]);
    let mut packets = vec![p1.clone(), p2.clone()];
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        if !line.is_empty() {
            let packet = line.parse::<Elem>()?;
            packets.push(packet)
        }
    }
    packets.sort();
    let idx = |p| packets.binary_search(&p)
        .map_err(|_| NotFound(p))
        .map(|i| i + 1);
    let idx1 = idx(p1.clone())?;
    let idx2 = idx(p2.clone())?;
    Ok(idx1 * idx2)
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
            }
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
    fn print_count_right_order() {
        println!("{}", count_right_order().unwrap())
    }

    #[test]
    fn print_decoder_key() {
        println!("{}", decoder_key().unwrap())
    }

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
