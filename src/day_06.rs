use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    MarkerNotFound,
}

fn prefix_length(marker_length: usize) -> Result<usize, Error> {
    let stream = read_to_string("input-06.txt").map_err(|e| IO(e))?;
    for i in marker_length..stream.len() {
        let set: HashSet<char> = HashSet::from_iter(stream[i - marker_length..i].chars());
        if set.len() == marker_length {
            return Ok(i);
        }
    }
    Err(MarkerNotFound)
}

fn packet_prefix_length() -> Result<usize, Error> {
    prefix_length(4)
}

fn message_prefix_length() -> Result<usize, Error> {
    prefix_length(14)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_packet_prefix_length() {
        println!("{}", packet_prefix_length().unwrap());
    }

    #[test]
    fn print_message_prefix_length() {
        println!("{}", message_prefix_length().unwrap());
    }
}

