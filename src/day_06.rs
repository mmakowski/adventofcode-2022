use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    MarkerNotFound,
}

fn prefix_length() -> Result<usize, Error> {
    const MARKER_LENGTH: usize = 4;
    let stream = read_to_string("input-06.txt").map_err(|e| IO(e))?;
    for i in MARKER_LENGTH..stream.len() {
        let set: HashSet<char> = HashSet::from_iter(stream[i - MARKER_LENGTH..i].chars());
        if set.len() == 4 {
            return Ok(i);
        }
    }
    Err(MarkerNotFound)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_prefix_length() {
        println!("{}", prefix_length().unwrap());
    }
}

