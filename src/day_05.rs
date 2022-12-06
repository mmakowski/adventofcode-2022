use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

use stacks::*;

use crate::day_05::Error::*;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    NotEnoughCrates(usize),
    MalformedMoveDesc(String),
    ParseInt(ParseIntError),
}

mod stacks {
    use crate::day_05::Error;
    use crate::day_05::Error::NotEnoughCrates;

    pub type Stack = Vec<char>;
    pub type Stacks = Vec<Stack>;

    #[derive(Debug)]
    pub struct Move {
        pub num_crates: usize,
        pub from_stack: usize,
        pub to_stack: usize,
    }

    pub fn make_move(stacks: &mut Stacks, mov: Move) -> Result<(), Error> {
        for _ in 0..mov.num_crates {
            let c = stacks[mov.from_stack].pop().ok_or(NotEnoughCrates(mov.from_stack))?;
            stacks[mov.to_stack].push(c);
        }
        Ok(())
    }

    pub fn make_move_9001(stacks: &mut Stacks, mov: Move) -> Result<(), Error> {
        let from = &mut stacks[mov.from_stack];
        let from_num_remaining = from.len() - mov.num_crates;
        let crates: Vec<char> = from.iter()
            .skip(from_num_remaining)
            .map(|c| c.to_owned())
            .collect();
        from.truncate(from_num_remaining);
        stacks[mov.to_stack].extend(crates);
        Ok(())
    }

    pub fn mk_stacks(num_stacks: usize) -> Stacks {
        (0..num_stacks).map(|_| Vec::new()).collect::<Stacks>()
    }

    pub fn add_crate_bottom(stacks: &mut Stacks, stack_num: usize, crate_char: char) {
        stacks[stack_num].insert(0, crate_char)
    }

    pub fn top_crates_str(stacks: &Stacks) -> String {
        stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect::<String>()
    }
}

enum InputSection {
    Crates,
    Separator,
    Moves,
}

fn parse_crates(line: &String, num_stacks: usize) -> Result<Vec<Option<char>>, Error> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut result = (0..num_stacks).map(|_| None).collect::<Vec<Option<char>>>();
    for i in 0..num_stacks {
        let crate_char = chars[i * 4 + 1];
        // TODO: sanity check crate_char
        if crate_char != ' ' {
            result[i] = Some(crate_char);
        }
    }
    Ok(result)
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: do not compile the regex on every invocation
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let cap = re.captures(s).ok_or(MalformedMoveDesc(String::from(s)))?;
        let parse_int = |i: usize| { cap[i].parse::<usize>().map_err(|e| ParseInt(e)) };
        let num_crates = parse_int(1)?;
        // we use base-0 stack indices but the description is base-1
        let from_stack = parse_int(2)? - 1;
        let to_stack = parse_int(3)? - 1;
        Ok(Move { num_crates, from_stack, to_stack })
    }
}

fn parse(input_file_path: &str) -> Result<(Stacks, Vec<Move>), Error> {
    // cheating a bit
    const NUM_STACKS: usize = 9;
    use InputSection::*;
    let mut section = Crates;
    let mut stacks: Stacks = mk_stacks(NUM_STACKS);
    let mut moves: Vec<Move> = Vec::new();
    let file = File::open(input_file_path).map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        match section {
            Crates => {
                if line.contains('[') {
                    let level = parse_crates(&line, NUM_STACKS)?;
                    for i in 0..NUM_STACKS {
                        match level[i] {
                            Some(crate_char) => add_crate_bottom(&mut stacks, i, crate_char),
                            None => ()
                        }
                    }
                } else {
                    section = Separator
                }
            }
            Separator => section = Moves,
            Moves => {
                moves.push(line.parse::<Move>()?)
            }
        }
    }
    Ok((stacks, moves))
}

fn top_crates() -> Result<String, Error> {
    let (mut stacks, moves) = parse("input-05.txt")?;
    // println!("{:?}", stacks);
    // println!("{:?}", moves);
    for mov in moves {
        make_move(&mut stacks, mov)?;
    }
    Ok(top_crates_str(&stacks))
}

fn top_crates_9001() -> Result<String, Error> {
    let (mut stacks, moves) = parse("input-05.txt")?;
    for mov in moves {
        make_move_9001(&mut stacks, mov)?;
    }
    Ok(top_crates_str(&stacks))
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_top_crates() {
        println!("{}", top_crates().unwrap());
    }

    #[test]
    fn print_top_crates_9001() {
        println!("{}", top_crates_9001().unwrap());
    }
}
