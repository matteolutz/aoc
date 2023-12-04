#[derive(Debug)]
pub struct ScratchCard {
    winning: Vec<u32>,
    own_numbers: Vec<u32>
}

impl ScratchCard {
    pub fn worth(&self) -> u32 {
        let num_winning = self.winning.iter().filter(|n| self.own_numbers.contains(n)).count() as u32;
        if num_winning > 0 { 2u32.pow(num_winning - 1) } else { 0 }
    }
}

#[aoc_generator(day4)]
pub fn input_cards(input: &str) -> Vec<ScratchCard> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers, own_numbers) = numbers.split_once(" | ").unwrap();
            ScratchCard {
                winning: winning_numbers.trim().split(" ").filter(|n| !n.is_empty()).map(|n| n.trim().parse().unwrap()).collect(),
                own_numbers: own_numbers.trim().split(" ").filter(|n| !n.is_empty()).map(|n| n.trim().parse().unwrap()).collect()
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
    0
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
}