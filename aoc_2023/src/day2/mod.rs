use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct GameRevealSet {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

impl GameRevealSet {
    fn new() -> Self {
        GameRevealSet { num_red: 0, num_green: 0, num_blue: 0 }
    }
}

impl FromStr for GameRevealSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reveal_set = Self::new();

        for r in s.split(", ") {
            let (num, color) = r.split_once(" ").unwrap();
            let num = u32::from_str(num).unwrap();
            match color {
                "red" => reveal_set.num_red = num,
                "green" => reveal_set.num_green = num,
                "blue" => reveal_set.num_blue = num,
                _ => panic!("Unknown color: {}", color)
            }
        }

        Ok(reveal_set)
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    revealed: Vec<GameRevealSet>,
}

#[aoc_generator(day2)]
pub fn input_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let (_, revealed_sets) = l.split_once(": ").unwrap();
            let reveals = revealed_sets
                .split("; ")
                .map(|s| GameRevealSet::from_str(s).unwrap())
                .collect();
            Game { revealed: reveals }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &[Game]) -> u32 {
    let mut total: u32 = 0;
    for (idx, g) in games.iter().enumerate() {
        if g.revealed.iter().any(|r| r.num_red > 12 || r.num_green > 13 || r.num_blue > 14) {
            continue;
        }

        total += (idx as u32) + 1;
    }

    total
}

#[aoc(day2, part2)]
pub fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            let min_red = g.revealed.iter().map(|r| r.num_red).max().unwrap();
            let min_green = g.revealed.iter().map(|r| r.num_green).max().unwrap();
            let min_blue = g.revealed.iter().map(|r| r.num_blue).max().unwrap();
            min_red * min_green * min_blue
        })
        .sum()

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let total = super::part1(&super::input_games(input));
        assert_eq!(total, 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let total = super::part2(&super::input_games(input));
        assert_eq!(total, 2286);
    }
}