use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

use Dir::*;
use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    NoToken(String),
    ParseInt(ParseIntError),
    UnrecognisedDir(String),
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn adjacent_to(self: &Pos, pos2: &Pos) -> bool {
        self.x.abs_diff(pos2.x) <= 1 &&
            self.y.abs_diff(pos2.y) <= 1
    }
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(UnrecognisedDir(s.to_string()))
        }
    }
}

fn move_head(pos: &Pos, dir: &Dir) -> Pos {
    match dir {
        Left => Pos { x: pos.x - 1, y: pos.y },
        Right => Pos { x: pos.x + 1, y: pos.y },
        Up => Pos { x: pos.x, y: pos.y + 1 },
        Down => Pos { x: pos.x, y: pos.y - 1 }
    }
}

fn move_tail(head_pos: &Pos, tail_pos: &Pos) -> Pos {
    if head_pos.adjacent_to(tail_pos) {
        tail_pos.clone()
    } else {
        Pos {
            x: tail_pos.x + (head_pos.x - tail_pos.x).signum(),
            y: tail_pos.y + (head_pos.y - tail_pos.y).signum(),
        }
    }
}

fn tail_locs(rope_length: usize) -> Result<usize, Error> {
    let file = File::open("input-09.txt").map_err(|e| IO(e))?;
    let head: usize = 0;
    let tail: usize = rope_length - 1;
    let mut rope_pos = vec![Pos { x: 0, y: 0 }; rope_length];
    let mut tail_locs: HashSet<Pos> = HashSet::from([rope_pos[tail]]);
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        let mut parts = line.split_whitespace();
        let dir = parts.next()
            .ok_or(NoToken(line.clone()))?
            .parse::<Dir>()?;
        let steps = parts.next()
            .ok_or(NoToken(line.clone()))?
            .parse::<u32>().map_err(|e| ParseInt(e))?;
        for _ in 0..steps {
            rope_pos[head] = move_head(&rope_pos[head], &dir);
            for i in 1..rope_length {
                rope_pos[i] = move_tail(&rope_pos[i - 1], &rope_pos[i]);
            }
            tail_locs.insert(rope_pos[tail]);
        }
    }
    Ok(tail_locs.len())
}

fn short_tail_locs() -> Result<usize, Error> {
    tail_locs(2)
}

fn long_tail_locs() -> Result<usize, Error> {
    tail_locs(10)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_short_tail_locs() {
        println!("{}", short_tail_locs().unwrap());
    }

    #[test]
    fn print_long_tail_locs() {
        println!("{}", long_tail_locs().unwrap());
    }
}