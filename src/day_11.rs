use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    InvalidState(u8),
    ParseInt(ParseIntError),
}

struct Monkey<'a> {
    items: Vec<u64>,
    // TODO: Box<dyn Fn(u64) -> u64>
    op: &'a dyn Fn(u64) -> u64,
    dest: &'a dyn Fn(u64) -> usize,
    inspection_count: u64,
}

impl<'a> Monkey<'a> {
    fn new(
        items: Vec<u64>,
        op: &'a dyn Fn(u64) -> u64,
        dest: &'a dyn Fn(u64) -> usize,
    ) -> Self {
        Self { items, op, dest, inspection_count: 0 }
    }
}

#[derive(Clone)]
struct ParserState<'a> {
    state: u8,
    items: Vec<u64>,
    op: &'a dyn Fn(u64) -> u64,
    div: u64,
    true_dest: usize,
    false_dest: usize,
}

impl<'a> ParserState<'a> {
    fn new() -> Self {
        ParserState {
            state: 0,
            items: vec![],
            op: &|_| { 0 },
            div: 0,
            true_dest: 0,
            false_dest: 0,
        }
    }

    fn next_line(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        let result = match self.state {
            0 | 6 => Ok(None), // skip
            1 => {
                todo!("parse items")
            }
            2 => {
                todo!("parse op")
            }
            3 => {
                todo!("parse div")
            }
            4 => {
                todo!("true dest")
            }
            5 => {
                todo!("false dest")
            }
            other => Err(InvalidState(other))
        };
        self.state = (self.state + 1) % 7;
        result
    }

    fn monkey(&self) -> Monkey {
        let x = |wl|
            if wl % self.div == 0 {
                self.true_dest
            } else {
                self.false_dest
            };
        // TODO: fix this (to do with closure refs)
        Monkey::new(
            self.items.clone(),
            self.op,
            &x,
        )
    }
}

fn parse(path: &str) -> Result<Vec<Monkey>, Error> {
    let mut result = vec![];
    let mut state = ParserState::new();
    let file = File::open(path).map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        if let Some(monkey) = state.next_line(&line)? {
            result.push(monkey);
        }
    }
    Ok(result)
}

fn monkey_business_level() -> Result<u64, Error> {
    let mut monkeys = parse("input-11.txt")?;
    // TODO
    println!("{:?}", monkeys.iter().map(|m| m.items.clone()).collect::<Vec<Vec<u64>>>());
    Ok(0)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_monkey_business_level() {
        println!("{}", monkey_business_level().unwrap());
    }
}