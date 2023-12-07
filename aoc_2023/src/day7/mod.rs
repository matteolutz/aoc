use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn from_cards(cards: &[Card], joker: bool) -> HandType {
        if joker {
            let joker_pos = cards.iter().position(|c| *c == Card::CJoker);

            if let Some(joker_pos) = joker_pos {
                let mut best_hand = HandType::HighCard;

                let mut cards = cards.to_vec();
                for i in 1..=13 {
                    cards[joker_pos] = Card::from(i);
                    let hand = HandType::from_cards(&cards, true);
                    if hand > best_hand {
                        best_hand = hand;
                    }
                }

                return best_hand;
            }
        }

        let mut counts = [0; 14];
        for card in cards {
            counts[usize::from(card)] += 1;
        }

        let mut max_count = 0;
        let mut second_max_count = 0;
        for count in counts.iter() {
            if *count > max_count {
                second_max_count = max_count;
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }

        if max_count == 5 {
            HandType::FiveOfAKind
        } else if max_count == 4 {
            HandType::FourOfAKind
        } else if max_count == 3 && second_max_count == 2 {
            HandType::FullHouse
        } else if max_count == 3 {
            HandType::ThreeOfAKind
        } else if max_count == 2 && second_max_count == 2 {
            HandType::TwoPair
        } else if max_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Card {
    CJoker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

// impl a trait for card to cast it to a usize
impl From<&Card> for usize {
    fn from(card: &Card) -> Self {
        match card {
            Card::CJoker => 0,
            Card::C2 => 1,
            Card::C3 => 2,
            Card::C4 => 3,
            Card::C5 => 4,
            Card::C6 => 5,
            Card::C7 => 6,
            Card::C8 => 7,
            Card::C9 => 8,
            Card::CT => 9,
            Card::CJ => 10,
            Card::CQ => 11,
            Card::CK => 12,
            Card::CA => 13,
        }
    }
}

impl From<usize> for Card {
    fn from(i: usize) -> Self {
        match i {
            0 => Card::CJoker,
            1 => Card::C2,
            2 => Card::C3,
            3 => Card::C4,
            4 => Card::C5,
            5 => Card::C6,
            6 => Card::C7,
            7 => Card::C8,
            8 => Card::C9,
            9 => Card::CT,
            10 => Card::CJ,
            11 => Card::CQ,
            12 => Card::CK,
            13 => Card::CA,
            _ => panic!("Invalid card index"),
        }
    }

}

impl Card {
    fn from_str(s: &str, joker: bool) -> Result<Self, ()> {
        match s {
            "2" => Ok(Card::C2),
            "3" => Ok(Card::C3),
            "4" => Ok(Card::C4),
            "5" => Ok(Card::C5),
            "6" => Ok(Card::C6),
            "7" => Ok(Card::C7),
            "8" => Ok(Card::C8),
            "9" => Ok(Card::C9),
            "T" => Ok(Card::CT),
            "J" => Ok(if joker { Card::CJoker } else { Card::CJ }),
            "Q" => Ok(Card::CQ),
            "K" => Ok(Card::CK),
            "A" => Ok(Card::CA),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }

        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].partial_cmp(&other.cards[i]);
            }
        }

        None
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


pub fn input_hands(input: &str, joker: bool) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let cards = hand.chars().map(|c| Card::from_str(&c.to_string(), joker).unwrap()).collect::<Vec<Card>>();
            (Hand {
                hand_type: HandType::from_cards(&cards, joker),
                cards
            }, bid)
        })
        .collect::<Vec<(Hand, u32)>>()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let mut hands = input_hands(input, false);
    hands.sort();

    hands.iter().enumerate().fold(0, |acc, (idx, (_, bid))| acc + bid * (idx as u32 + 1))
}


#[aoc(day7, part2)]
pub fn part2(input: &str) -> u32 {
    let mut hands = input_hands(input, true);
    hands.sort();

    hands.iter().enumerate().fold(0, |acc, (idx, (_, bid))| acc + bid * (idx as u32 + 1))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hand_type() {
        assert!(super::HandType::HighCard < super::HandType::OnePair);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(super::part1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 6440);
    }


    #[test]
    pub fn test_part2() {
        assert_eq!(super::part2("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 5905);
    }
}