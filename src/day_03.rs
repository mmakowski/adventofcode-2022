use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
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

fn intersection(team: &Vec<String>) -> Result<char, String> {
    let common_chars = team.iter()
        .map(|i| HashSet::<char>::from_iter(i.chars()))
        .reduce(|acc, item| {
            acc.intersection(&item).map(|c| c.to_owned()).collect::<HashSet<char>>()
        })
        .unwrap();
    assert_eq!(common_chars.len(), 1);
    Ok(common_chars.iter().next().unwrap().to_owned())
}

fn team_overlap() -> io::Result<u64> {
    let mut sum: u64 = 0;
    let file = File::open("input-03.txt")?;
    const TEAM_SIZE: usize = 3;
    let mut team: Vec<String> = (0..TEAM_SIZE).map(|_| String::new()).collect();
    let mut i: usize = 0;
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        team[i] = line;
        i += 1;
        if i == TEAM_SIZE {
            sum += priority(intersection(&team).unwrap()).unwrap();
            i = 0;
        }
   }
    Ok(sum)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_compartment_overlap() {
        println!("{}", compartment_overlap().unwrap());
    }

    #[test]
    fn print_team_overlap() {
        println!("{}", team_overlap().unwrap());
    }
}
