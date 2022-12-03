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

fn sum_of_priorities() -> io::Result<u64> {
    let mut sum: u64 = 0;
    let file = File::open("input-03.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let compartment_size = l.len() / 2;
            let c1: HashSet<char> = HashSet::from_iter(l.get(0..compartment_size).unwrap().chars());
            let c2: HashSet<char> = HashSet::from_iter(l.get(compartment_size..l.len()).unwrap().chars());
            let intersection = c1.intersection(&c2);
            // TODO: clean this int2 mess up
            let mut int2 = intersection.clone();
            assert_eq!(intersection.count(), 1);
            let common_elem = int2.next().unwrap().clone();
            sum += priority(common_elem).unwrap();
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_sum_of_priorities() {
        println!("{}", sum_of_priorities().unwrap());
    }
}
