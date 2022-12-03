mod day_1 {
    use std::fs::File;
    use std::io::{self, BufRead};

    fn max_calories() -> io::Result<u64> {
        let mut max: u64 = 0;
        let mut curr: u64 = 0;
        let file = File::open("input-01.txt")?;
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                if l.is_empty() {
                    if curr > max {
                        max = curr;
                    }
                    curr = 0;
                } else {
                    curr += l.parse::<u64>().unwrap();
                }
            }
        }
        if curr > max {
            max = curr;
        }
        Ok(max)
    }

    fn top_three_calories() -> io::Result<u64> {
        let mut totals: Vec<u64> = Vec::new();
        let mut curr: u64 = 0;
        let file = File::open("input-01.txt")?;
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                if l.is_empty() {
                    totals.push(curr);
                    curr = 0;
                } else {
                    curr += l.parse::<u64>().unwrap();
                }
            }
        }
        totals.push(curr);
        totals.sort();
        totals.reverse();
        let top3_sum = totals[0] + totals[1] + totals[2];
        Ok(top3_sum)
    }

    #[cfg(test)]
    mod run {
        use super::*;

        #[test]
        fn print_max_calories() {
            println!("{}", max_calories().unwrap());
        }

        #[test]
        fn print_top_three_calories() {
            println!("{}", top_three_calories().unwrap());
        }
    }
}

mod day_2 {
    use std::fs::File;
    use std::io::{self, BufRead};

    mod rps {
        pub enum Move {
            Rock, Paper, Scissors
        }

        pub fn my_score(my_move: Move, their_move: Move) -> u64 {
            use Move::*;
            let shape_score = match my_move {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };
            let fight_score = match (my_move, their_move) {
                (Rock, Rock) => 3,
                (Rock, Paper) => 0,
                (Rock, Scissors) => 6,
                (Paper, Rock) => 6,
                (Paper, Paper) => 3,
                (Paper, Scissors) => 0,
                (Scissors, Rock) => 0,
                (Scissors, Paper) => 6,
                (Scissors, Scissors) => 3,
            };
            shape_score + fight_score
        }
    }

    fn decode(code: &str) -> Result<rps::Move, &str> {
        use rps::Move::*;
        match code {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(code),
        }
    }

    fn score() -> io::Result<u64> {
        let file = File::open("input-02.txt")?;
        let mut total: u64 = 0;
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                let mut parts = l.split_whitespace();
                let their_move = parts.next().unwrap();
                let my_move = parts.next().unwrap();
                total += rps::my_score(decode(my_move).unwrap(), decode(their_move).unwrap());
            }
        }
        Ok(total)
    }

    #[cfg(test)]
    mod run {
        use super::*;

        #[test]
        fn print_score() {
            println!("{}", score().unwrap());
        }
    }
}
