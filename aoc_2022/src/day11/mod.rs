use std::cell::RefCell;

const PART_1_TOTAL_ROUNDS: u32 = 20;
const PART_2_TOTAL_ROUNDS: u32 = 10_000;

#[derive(Debug, Clone)]
pub enum Operation {
    MultiplySelf,
    Multiply(u128),
    AddSelf,
    Add(u128),
}

#[derive(Debug, Clone)]
pub struct Test {
    divisible_by: u128,
    receiver_if_true: usize,
    receiver_if_false: usize,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    inspections: u128,
}

#[aoc_generator(day11)]
pub fn input_monkey_data(input: &str) -> Vec<RefCell<Monkey>> {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let lines = l.lines().collect::<Vec<&str>>();
            let items = lines[1]
                .split_once(':')
                .unwrap().1
                .split(", ")
                .map(|i| i.trim().parse::<u128>().unwrap())
                .collect::<Vec<u128>>();

            let operation_parts = lines[2].split_once('=').unwrap().1.trim()
                .split_whitespace().collect::<Vec<&str>>();
            let operation_fn = match operation_parts[1] {
                "+" => {
                    if operation_parts[2] == "old" {
                        Operation::AddSelf
                    } else {
                        let val = operation_parts[2].parse::<u128>().unwrap();
                        Operation::Add(val)
                    }
                }
                "*" => {
                    if operation_parts[2] == "old" {
                        Operation::MultiplySelf
                    } else {
                        let val = operation_parts[2].parse::<u128>().unwrap();
                        Operation::Multiply(val)
                    }
                }
                _ => panic!("Unknown operation"),
            };

            let test_fn = Test {
                divisible_by: lines[3].split_once("by").unwrap().1.trim().parse::<u128>().unwrap(),
                receiver_if_true: lines[4].split_once("monkey").unwrap().1.trim().parse::<usize>().unwrap(),
                receiver_if_false: lines[5].split_once("monkey").unwrap().1.trim().parse::<usize>().unwrap(),
            };

            RefCell::new(Monkey {
                items,
                operation: operation_fn,
                test: test_fn,
                inspections: 0,
            })
        })
        .collect::<Vec<RefCell<Monkey>>>()
}

#[aoc(day11, part1)]
pub fn part1(input: &[RefCell<Monkey>]) -> u128 {
    let mut monkeys = input.to_vec();

    for _ in 0..PART_1_TOTAL_ROUNDS {
        for m in monkeys.iter() {
            let mut items = m.borrow().items.clone();
            items.reverse();

            for _ in 0..items.len() {
                let mut i = items.pop().unwrap();

                m.borrow_mut().inspections += 1;

                i = match &m.borrow().operation {
                    Operation::MultiplySelf => i.pow(2),
                    Operation::Multiply(val) => i * val,
                    Operation::AddSelf => i.pow(2),
                    Operation::Add(val) => i + val,
                } / 3;

                if i % *&m.borrow().test.divisible_by == 0 {
                    monkeys[*&m.borrow().test.receiver_if_true].borrow_mut().items.push(i);
                } else {
                    monkeys[*&m.borrow().test.receiver_if_false].borrow_mut().items.push(i);
                }
            }

            m.borrow_mut().items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.borrow().inspections.cmp(&a.borrow().inspections));

    let res = *&monkeys[0].borrow().inspections * *&monkeys[1].borrow().inspections;
    res
}

#[aoc(day11, part2)]
pub fn part2(input: &[RefCell<Monkey>]) -> u128 {
    let mut monkeys = input.to_vec();

    for _ in 0..PART_2_TOTAL_ROUNDS {
        for m in monkeys.iter() {
            let mut items = m.borrow().items.clone();
            items.reverse();

            for _ in 0..items.len() {
                let mut i = items.pop().unwrap();

                m.borrow_mut().inspections += 1;

                i = match &m.borrow().operation {
                    Operation::MultiplySelf => i * i,
                    Operation::Multiply(val) => i * val,
                    Operation::AddSelf => i * i,
                    Operation::Add(val) => i + val,
                };

                if i % *&m.borrow().test.divisible_by == 0 {
                    monkeys[*&m.borrow().test.receiver_if_true].borrow_mut().items.push(i);
                } else {
                    monkeys[*&m.borrow().test.receiver_if_false].borrow_mut().items.push(i);
                }
            }

            m.borrow_mut().items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.borrow().inspections.cmp(&a.borrow().inspections));

    let res = *&monkeys[0].borrow().inspections * *&monkeys[1].borrow().inspections;
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_part2() {
        println!("{:?}", part2(&input_monkey_data("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1")))
    }
}