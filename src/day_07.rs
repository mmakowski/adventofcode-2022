use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;

use Command::*;
use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    EmptyCommand,
    UnrecognisedCommand(String),
    MissingArg(String),
    EmptyLine,
    ParseInt(ParseIntError),
}

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

fn parse_command(line: &String) -> Result<Command, Error> {
    let mut parts = line.split_whitespace();
    parts.next();
    match parts.next().ok_or(EmptyCommand)? {
        "cd" => parts.next().map(|dir| Cd(dir)).ok_or(MissingArg(line.to_string())),
        "ls" => Ok(Ls),
        other => Err(UnrecognisedCommand(other.to_string()))
    }
}

fn parse_ls_entry_size(line: &String) -> Result<u64, Error> {
    let mut parts = line.split_whitespace();
    match parts.next().ok_or(EmptyLine)? {
        "dir" => Ok(0),
        num => num.parse::<u64>().map_err(|e| ParseInt(e))
    }
}

fn parse(path: &str) -> Result<HashMap<Vec<String>, u64>, Error> {
    let mut result = HashMap::new();
    let file = File::open(path).map_err(|e| IO(e))?;
    let mut cwd: Vec<String> = Vec::new();
    let mut cwd_size: u64 = 0;
    let mut finish_ls = |mut cwd: Vec<String>, cwd_size: u64| {
        if cwd_size > 0 {
            loop {
                let prev_size = result.get(&cwd).unwrap_or(&0);
                result.insert(cwd.clone(), cwd_size + prev_size);
                if cwd.is_empty() { break; }
                cwd.pop();
            }
        }
    };
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        if line.starts_with("$") {
            finish_ls(cwd.clone(), cwd_size);
            cwd_size = 0;
            match parse_command(&line)? {
                Ls => (),
                Cd("/") => cwd.clear(),
                Cd("..") => {
                    cwd.pop();
                    ()
                }
                Cd(dir) => cwd.push(String::from(dir))
            }
        } else {
            cwd_size += parse_ls_entry_size(&line)?;
        }
    }
    finish_ls(cwd.clone(), cwd_size);
    Ok(result)
}

fn small_dirs_size() -> Result<u64, Error> {
    let dir_map = parse("input-07.txt")?;
    //println!("{:?}", dir_map);
    Ok(dir_map.values().filter(|v| v <= &&100_000).sum())
}

fn space_to_free() -> Result<u64, Error> {
    let dir_map = parse("input-07.txt")?;
    const DISK_SIZE: u64 = 70_000_000;
    const REQUIRED_SPACE: u64 = 30_000_000;
    let free_space = DISK_SIZE - dir_map[&vec![]];
    let to_free = REQUIRED_SPACE - free_space;
    Ok(dir_map.values().filter(|v| v >= &&to_free).min().unwrap().to_owned())
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_small_dirs_size() {
        println!("{}", small_dirs_size().unwrap());
    }

    #[test]
    fn print_space_to_free() {
        println!("{}", space_to_free().unwrap());
    }
}
