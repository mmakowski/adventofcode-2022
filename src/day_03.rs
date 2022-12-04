use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn priority(c: char) -> Result<u64, char> {
    if c.is_ascii_lowercase() {
        Ok(((c as u8) - ('a' as u8) + 1) as u64)
    } else if c.is_ascii_uppercase() {
            Ok(((c as u8) - ('A' as u8) + 27) as u64)
    } else {
        Err(c)
    }
}

fn compartment_overlap() -> io::Result<u64> {
    let mut sum: u64 = 0;
    let file = File::open("input-03.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let compartment_size = l.len() / 2;
            let c1: HashSet<char> = HashSet::from_iter(l.get(0..compartment_size).unwrap().chars());
            let c2: HashSet<char> = HashSet::from_iter(l.get(compartment_size..l.len()).unwrap().chars());
            let mut intersection = c1.intersection(&c2);
            let common_elem = intersection.next().unwrap().clone();
            assert!(intersection.next().is_none());
            sum += priority(common_elem).unwrap();
        }
    }
    Ok(sum)
}

fn threes_overlap() -> io::Result<u64> {
    let mut sum: u64 = 0;
    let file = File::open("input-03.txt")?;
    for line in io::BufReader::new(file).lines() {
        // TODO: chunks of 3 lines
    }
    Ok(0)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_compartment_overlap() {
        println!("{}", compartment_overlap().unwrap());
    }

    #[test]
    fn print_threes_overlap() {
        println!("{}", threes_overlap().unwrap());
    }
}
