use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ScratchCard {
    game_idx: usize,
    winning: HashSet<u32>,
    own_numbers: HashSet<u32>,
}

impl ScratchCard {
    pub fn num_winning(&self) -> usize {
        self.winning.intersection(&self.own_numbers).count()
    }

    pub fn worth(&self) -> u32 {
        let num_winning = self.num_winning();
        if num_winning > 0 { 2u32.pow(num_winning as u32 - 1) } else { 0 }
    }
}

#[aoc_generator(day4)]
pub fn input_cards(input: &str) -> Vec<ScratchCard> {
    input
        .lines()
        .map(|line| {
            let (game, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers, own_numbers) = numbers.split_once(" | ").unwrap();
            ScratchCard {
                winning: winning_numbers.trim().split_whitespace().filter(|n| !n.is_empty()).map(|n| n.trim().parse().unwrap()).collect(),
                own_numbers: own_numbers.trim().split_whitespace().filter(|n| !n.is_empty()).map(|n| n.trim().parse().unwrap()).collect(),
                game_idx: game.split_once(" ").unwrap().1.trim().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[ScratchCard]) -> u32 {
    input.iter().map(|card| card.worth()).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[ScratchCard]) -> u32 {
    let mut cards = input.iter().map(|c| (c.game_idx, 1usize)).collect::<HashMap<usize, usize>>();

    for i in 0..input.len() {
        for j in 0..input[i].num_winning() {
            let d = *cards.get(&input[i].game_idx).unwrap();
            let count = cards.entry(input[i].game_idx + j + 1).or_insert(0);
            *count += d;
        }
    }

    cards.iter().map(|(_, v)| *v).sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use crate::day4::input_cards;

    #[test]
    fn test_part_1() {
        let input = input_cards("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(super::part1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = input_cards("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(super::part2(&input), 30);
    }
}