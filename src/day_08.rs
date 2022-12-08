use std::fs::File;
use std::io;
use std::io::BufRead;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error)
}

fn parse(line: &str) -> Result<Vec<u8>, Error> {
    let mut result: Vec<u8> = Vec::new();
    for c in line.chars() {
        result.push(c as u8)
    }
    Ok(result)
}

fn read(path: &str) -> Result<Vec<Vec<u8>>, Error> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let file = File::open(path).map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        result.push(parse(&line)?)
    }
    Ok(result)
}

fn tree_cover() -> Result<usize, Error> {
    let trees = read("input-08.txt")?;
    let width = trees[0].len();
    let height = trees.len();
    let mut mask: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width]).collect();

    // scans
    //  top-down
    let mut row_max: Vec<u8> = vec![0; width];
    for row in 0..height {
        for col in 0..width {
            if trees[col][row] > row_max[col] {
                row_max[col] = trees[col][row];
                mask[col][row] = true
            }
        }
    }
    //  bottom-up
    let mut row_max: Vec<u8> = vec![0; width];
    for row in (0..height).rev() {
        for col in 0..width {
            if trees[col][row] > row_max[col] {
                row_max[col] = trees[col][row];
                mask[col][row] = true
            }
        }
    }
    //  left-right
    let mut col_max: Vec<u8> = vec![0; height];
    for col in 0..width {
        for row in 0..height {
            if trees[col][row] > col_max[row] {
                col_max[row] = trees[col][row];
                mask[col][row] = true
            }
        }
    }
    //  right-left
    let mut col_max: Vec<u8> = vec![0; height];
    for col in (0..width).rev() {
        for row in 0..height {
            if trees[col][row] > col_max[row] {
                col_max[row] = trees[col][row];
                mask[col][row] = true
            }
        }
    }

    // for row in 0..height {
    //     for col in 0..width {
    //         let c = if mask[col][row] { "X" } else { "." };
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let num_visible = mask.iter().map(|row|
        row.iter().filter(|v| v == &&true).count()
    ).sum();
    Ok(num_visible)
}


#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_tree_cover() {
        println!("{}", tree_cover().unwrap());
    }
}