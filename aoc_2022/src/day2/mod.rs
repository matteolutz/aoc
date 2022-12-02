#[derive(Copy, Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone, Debug)]
pub struct Strategy {
    opponent: Move,
    my_move: Move,
}

#[aoc_generator(day2)]
pub fn input_strategies(input: &str) -> Vec<Strategy> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<&str>>();
            let opponent_move = match parts[0] {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Invalid move"),
            };
            let my_moe = match parts[1] {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!("Invalid move"),
            };
            Strategy {
                opponent: opponent_move,
                my_move: my_moe,
            }
        }).collect()
}

pub fn calculate_round(strategy: &Strategy) -> i32 {
    // TODO
    0
}

#[aoc(day2, part1)]
pub fn part1(strategies: &[Strategy]) -> i32 {
    0
}