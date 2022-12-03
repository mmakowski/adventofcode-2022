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
    use std::str::FromStr;

    mod rps {
        #[derive(Clone, Copy)]
        pub enum Move {
            Rock,
            Paper,
            Scissors,
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

    use rps::Move::*;

    impl FromStr for rps::Move {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Rock),
                "B" => Ok(Paper),
                "C" => Ok(Scissors),
                "X" => Ok(Rock),
                "Y" => Ok(Paper),
                "Z" => Ok(Scissors),
                _ => Err(String::from(s)),
            }
        }
    }

    fn score_with_assumption() -> io::Result<u64> {
        let file = File::open("input-02.txt")?;
        let mut total: u64 = 0;
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                let mut parts = l.split_whitespace();
                let mut next_move = || { parts.next().unwrap().parse::<rps::Move>().unwrap() };
                let their_move = next_move();
                let my_move = next_move();
                total += rps::my_score(my_move, their_move);
            }
        }
        Ok(total)
    }

    enum RoundResult {
        Win,
        Draw,
        Lose,
    }

    impl FromStr for RoundResult {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use RoundResult::*;
            match s {
                "X" => Ok(Lose),
                "Y" => Ok(Draw),
                "Z" => Ok(Win),
                _ => Err(String::from(s))
            }
        }
    }

    fn choose_move(their_move: rps::Move, intended_result: RoundResult) -> rps::Move {
        use RoundResult::*;
        match their_move {
            Rock => match intended_result {
                Win => Paper,
                Draw => Rock,
                Lose => Scissors,
            },
            Paper => match intended_result {
                Win => Scissors,
                Draw => Paper,
                Lose => Rock,
            },
            Scissors => match intended_result {
                Win => Rock,
                Draw => Scissors,
                Lose => Paper,
            },
        }
    }

    fn score_with_full_info() -> io::Result<u64> {
        let file = File::open("input-02.txt")?;
        let mut total: u64 = 0;
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                let mut parts = l.split_whitespace();
                let their_move = parts.next().unwrap().parse::<rps::Move>().unwrap();
                let intended_result = parts.next().unwrap().parse::<RoundResult>().unwrap();
                let my_move = choose_move(their_move, intended_result);
                total += rps::my_score(my_move, their_move);
            }
        }
        Ok(total)
    }


    #[cfg(test)]
    mod run {
        use super::*;

        #[test]
        fn print_score_with_assumption() {
            println!("{}", score_with_assumption().unwrap());
        }

        #[test]
        fn print_score_with_full_info() {
            println!("{}", score_with_full_info().unwrap());
        }
    }
}
