#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
pub enum PlayerMove {
    // ROCk
    X,

    // PAPER
    Y,

    // SCISSORS
    Z
}

#[derive( Debug)]
pub struct Strategy {
    opponent_move: Move,
    my_move_or_outcome: PlayerMove,
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
            let my_move = match parts[1] {
                "X" => PlayerMove::X,
                "Y" => PlayerMove::Y,
                "Z" => PlayerMove::Z,
                _ => panic!("Invalid move"),
            };
            Strategy {
                opponent_move: opponent_move,
                my_move_or_outcome: my_move,
            }
        }).collect()
}

pub fn get_outcome(my_move: &PlayerMove, opponent_move: &Move) -> i32 {
    match opponent_move {
        Move::Rock => match my_move {
            PlayerMove::X => 0,
            PlayerMove::Y => 1,
            PlayerMove::Z => -1,
        },
        Move::Paper => match my_move {
            PlayerMove::X => -1,
            PlayerMove::Y => 0,
            PlayerMove::Z => 1,
        },
        Move::Scissors => match my_move {
            PlayerMove::X => 1,
            PlayerMove::Y => -1,
            PlayerMove::Z => 0,
        },
    }
}

pub fn calculate_round(my_move: &PlayerMove, opponent_move: &Move) -> i32 {
    let shape_score = match my_move {
        PlayerMove::X => 1,
        PlayerMove::Y => 2,
        PlayerMove::Z => 3,
    };
    let outcome_score = match get_outcome(my_move, opponent_move) {
        1 => 6,
        0 => 3,
        -1 => 0,
        _ => panic!("Invalid outcome"),
    };
    shape_score + outcome_score
}

#[aoc(day2, part1)]
pub fn part1(strategies: &[Strategy]) -> i32 {
    strategies.iter()
        .fold(0, |acc, strategy| acc + calculate_round(&strategy.my_move_or_outcome, &strategy.opponent_move))
}

pub fn get_loose_move(m: &Move) -> PlayerMove {
    match m {
        Move::Rock => PlayerMove::Z,
        Move::Paper => PlayerMove::X,
        Move::Scissors => PlayerMove::Y,
    }
}

pub fn get_tie_move(m: &Move) -> PlayerMove {
    match m {
        Move::Rock => PlayerMove::X,
        Move::Paper => PlayerMove::Y,
        Move::Scissors => PlayerMove::Z,
    }
}

pub fn get_win_move(m: &Move) -> PlayerMove {
    match m {
        Move::Rock => PlayerMove::Y,
        Move::Paper => PlayerMove::Z,
        Move::Scissors => PlayerMove::X,
    }
}

#[aoc(day2, part2)]
pub fn part2(strategies: &[Strategy]) -> i32 {
    strategies.iter()
        .fold(0, |acc, strategy| {
            acc + match strategy.my_move_or_outcome {
                PlayerMove::X => calculate_round(&get_loose_move(&strategy.opponent_move), &strategy.opponent_move),
                PlayerMove::Y => calculate_round(&get_tie_move(&strategy.opponent_move), &strategy.opponent_move),
                PlayerMove::Z => calculate_round(&get_win_move(&strategy.opponent_move), &strategy.opponent_move),
            }
        })
}