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
        result.push(c as u8 - '0' as u8)
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
    let mut mask: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // scans
    //  top-down
    let mut row_max: Vec<u8> = vec![0; width];
    for row in 0..height {
        for col in 0..width {
            if trees[col][row] > row_max[col] { // TODO: indices are swapped here?
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

fn scenic_score() -> Result<u64, Error> {
    let trees = read("input-08.txt")?;
    let width = trees[0].len();
    let height = trees.len();

    let mut view_dists: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; 4]; width]; height];

    // TODO: scan in each dir, keeping track of the coord of most recent tree >= x for x in 0..9

    //  top-down
    let mut dir = 0;
    let mut height_coords: Vec<Vec<usize>> = vec![vec![0; 10]; width];
    for row in 0..height {
        for col in 0..width {
            let tree_height = trees[row][col] as usize;
            view_dists[row][col][dir] = row.abs_diff(height_coords[col][tree_height]) as u64;
            for h in 0..=tree_height {
                height_coords[col][h] = row
            }
        }
    }
    //  bottom-up
    dir += 1;
    let mut height_coords: Vec<Vec<usize>> = vec![vec![height-1; 10]; width];
    for row in (0..height).rev() {
        for col in 0..width {
            let tree_height = trees[row][col] as usize;
            view_dists[row][col][dir] = row.abs_diff(height_coords[col][tree_height]) as u64;
            for h in 0..=tree_height {
                height_coords[col][h] = row
            }
        }
    }
    //  left-right
    dir += 1;
    let mut height_coords: Vec<Vec<usize>> = vec![vec![0; 10]; height];
    for col in 0..width {
        for row in 0..height {
            let tree_height = trees[row][col] as usize;
            view_dists[row][col][dir] = col.abs_diff(height_coords[row][tree_height]) as u64;
            for h in 0..=tree_height {
                height_coords[row][h] = col
            }
        }
    }
    //  right-left
    dir += 1;
    let mut height_coords: Vec<Vec<usize>> = vec![vec![width-1; 10]; height];
    for col in (0..width).rev() {
        for row in 0..height {
            let tree_height = trees[row][col] as usize;
            view_dists[row][col][dir] = col.abs_diff(height_coords[row][tree_height]) as u64;
            for h in 0..=tree_height {
                height_coords[row][h] = col
            }
        }
    }


    let best_score = view_dists.iter().flat_map(|row|
        row.iter().map(|dists| dists.iter().product::<u64>()).max()
    ).max().unwrap();
    Ok(best_score)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_tree_cover() {
        println!("{}", tree_cover().unwrap());
    }

    #[test]
    fn print_scenic_score() {
        println!("{}", scenic_score().unwrap());
    }
}