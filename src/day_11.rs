use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

use num::bigint::BigUint;
use num::traits::Zero;
use regex::Regex;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    InvalidState(u8),
    ParseInt(ParseIntError),
    MalformedStartingItems(String),
    MalformedOperation(String),
    UnsupportedOperation(String),
    MalformedDiv(String),
    MalformedDest(String),
}

struct Monkey {
    // TODO: BigUints are super-slow, instead we can track the remainders for 1..20 for each item
    items: Vec<BigUint>,
    op: Box<dyn Fn(BigUint) -> BigUint>,
    dest: Box<dyn Fn(BigUint) -> usize>,
    inspection_count: u64,
}

impl Monkey {
    fn new(
        items: Vec<BigUint>,
        op: Box<dyn Fn(BigUint) -> BigUint>,
        dest: Box<dyn Fn(BigUint) -> usize>,
    ) -> Self {
        Self { items, op, dest, inspection_count: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            other => Err(UnsupportedOperation(other.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Num(u32),
    Old
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Operand::Old)
        } else {
            let x = s.parse::<u32>().map_err(ParseInt)?;
            Ok(Operand::Num(x))
        }
    }
}

#[derive(Debug, Clone)]
struct ParserState {
    state: u8,
    items: Vec<BigUint>,
    operation: Operation,
    operand: Operand,
    div: u32,
    true_dest: usize,
    false_dest: usize,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState {
            state: 0,
            items: vec![],
            operation: Operation::Add,
            operand: Operand::Old,
            div: 0,
            true_dest: 0,
            false_dest: 0,
        }
    }

    pub fn next_line(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        let result = match self.state {
            0 | 6 => Ok(None), // skip
            1 => self.parse_items(line),
            2 => self.parse_op(line),
            3 => self.parse_div(line),
            4 => self.parse_true_dest(line),
            5 => self.parse_false_dest(line),
            other => Err(InvalidState(other))
        };
        self.state = (self.state + 1) % 7;
        result
    }

    fn parse_items(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        let re = Regex::new(r"\s*Starting items: (((\d+)(,\s*)?)*)").unwrap();
        let cap = re.captures(line).ok_or(MalformedStartingItems(line.to_string()))?;
        let mut vals = vec![];
        for s in cap[1].split(", ") {
            vals.push(s.parse::<u32>().map_err(ParseInt).map(BigUint::from)?)
        }
        self.items = vals;
        Ok(None)
    }

    fn parse_op(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        let re = Regex::new(r"\s*Operation: new\s*=\s*old\s*(\+|\*)\s*(old|\d+)").unwrap();
        let cap = re.captures(line).ok_or(MalformedOperation(line.to_string()))?;
        let operation = cap[1].parse::<Operation>()?;
        let operand = cap[2].parse::<Operand>()?;
        self.operation = operation;
        self.operand = operand;
        Ok(None)
    }

    fn parse_div(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        let re = Regex::new(r"\s*Test: divisible by (\d+)").unwrap();
        let cap = re.captures(line).ok_or(MalformedDiv(line.to_string()))?;
        self.div = cap[1].parse::<u32>().map_err(ParseInt)?;
        Ok(None)
    }

    fn parse_true_dest(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        self.true_dest = self.parse_dest(line, "true")?;
        Ok(None)
    }

    fn parse_false_dest(&mut self, line: &String) -> Result<Option<Monkey>, Error> {
        self.false_dest = self.parse_dest(line, "false")?;
        Ok(Some(self.monkey()))
    }

    fn parse_dest(&mut self, line: &String, branch: &str) -> Result<usize, Error> {
        let re = Regex::new(format!(r"\s*If {}: throw to monkey (\d+)", branch).as_str()).unwrap();
        let cap = re.captures(line).ok_or(MalformedDest(line.to_string()))?;
        cap[1].parse::<usize>().map_err(ParseInt)
    }

    fn monkey(&self) -> Monkey {
        let operation = self.operation.to_owned();
        let operand = self.operand.to_owned();
        let op = move |wl: BigUint| {
            let o = match operand {
                Operand::Num(x) => BigUint::from(x),
                Operand::Old => wl.clone()
            };
            match operation {
                Operation::Add => wl + o,
                Operation::Mul => wl * o
            }
        };
        let div = self.div.to_owned();
        let true_dest = self.true_dest.to_owned();
        let false_dest = self.false_dest.to_owned();
        let dest = move |wl: BigUint|
            if wl % div == Zero::zero() {
                true_dest
            } else {
                false_dest
            };
        Monkey::new(
            self.items.clone(),
            Box::new(op),
            Box::new(dest),
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

fn monkey_business_level(div: u32, num_rounds: u16) -> Result<u64, Error> {
    let mut monkeys = parse("input-11.txt")?;
    let num_monkeys = monkeys.len();
    for _round in 0..num_rounds {
        for m in 0..num_monkeys {
            let monkey = &mut monkeys[m];
            let mut thrown: Vec<Vec<BigUint>> = vec![vec![]; num_monkeys];
            for item in &monkey.items {
                let mut wl = (monkey.op)(item.to_owned());
                monkey.inspection_count += 1;
                wl /= div;
                let dest = (monkey.dest)(wl.clone());
                thrown[dest].push(wl);
                // println!("[{}]: ({} -> {}) => [{}]", m, item, wl, dest);
            }
            monkey.items.clear();
            for om in 0..num_monkeys {
                monkeys[om].items.append( thrown[om].as_mut());
            }
        }
    }
    let mut counts: Vec<u64> = monkeys.iter()
        .map(|m| m.inspection_count)
        .collect();
    counts.sort();
    counts.reverse();
    // println!("{:?}", counts);
    Ok(counts[0] * counts[1])
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_monkey_business_level_div_3() {
        println!("{}", monkey_business_level(3, 20).unwrap());
    }

    #[test]
    fn print_monkey_business_level_no_div() {
        println!("{}", monkey_business_level(1, 10000).unwrap());
    }
}