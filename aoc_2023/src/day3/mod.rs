type EngineSchematic = Vec<Vec<char>>;

#[aoc_generator(day3)]
pub fn input_schematic(input: &str) -> EngineSchematic {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn is_symbol(c: &char) -> bool {
    let res = !c.is_alphabetic() && !c.is_numeric() && *c != '.';
    res
}

pub fn has_adjacent_symbol(schematic: &EngineSchematic, x: usize, y: usize) -> (bool, usize, usize) {
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let x = x as i32 + dx;
            let y = y as i32 + dy;
            if x < 0 || y < 0 || y >= schematic.len() as i32 || x >= schematic[y as usize].len() as i32 {
                continue;
            }

            if is_symbol(&schematic[y as usize][x as usize]) {
                return (true, x as usize, y as usize);
            }
        }
    }

    (false, 0, 0)
}

pub fn find_adjacent_numbers(schematic: &EngineSchematic, x: usize, y: usize) -> Vec<u32> {
    let mut numbers = Vec::<u32>::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let x = x as i32 + dx;
            let y = y as i32 + dy;
            if x < 0 || y < 0 || y >= schematic.len() as i32 || x >= schematic[y as usize].len() as i32 {
                continue;
            }

            if schematic[y as usize][x as usize].is_numeric() {
                let (number, _, _) = get_number(schematic, x as usize, y as usize);
                if numbers.contains(&number) {
                    // a bit of a hack, but it works
                    continue;
                }
                numbers.push(number);
            }
        }
    }

    numbers
}

pub fn get_number(schematic: &EngineSchematic, x: usize, y: usize) -> (u32, usize, usize) {
    let mut number = 0_u32;
    let mut first_digit_x = x;

    while first_digit_x > 0 && schematic[y][first_digit_x - 1].is_numeric() {
        first_digit_x -= 1;
    }

    let mut current_digit_x = first_digit_x;
    while current_digit_x < schematic[y].len() && schematic[y][current_digit_x].is_numeric() {
        number = number * 10 + schematic[y][current_digit_x].to_digit(10).unwrap();
        current_digit_x += 1;
    }

    (number, first_digit_x, y)
}

#[aoc(day3, part1)]
pub fn part1(schematic: &EngineSchematic) -> u32 {
    let mut total = 0_u32;

    let mut current_number: Option<u32> = None;
    let mut has_current_num_adjacent_symbol = false;
    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_numeric() {
                if current_number.is_none() {
                    current_number = Some(c.to_digit(10).unwrap());
                } else {
                    current_number = Some(current_number.unwrap() * 10 + c.to_digit(10).unwrap());
                }

                let (adjacent, _, _) = has_adjacent_symbol(schematic, x, y);
                has_current_num_adjacent_symbol |= adjacent;

                continue;
            }

            if let Some(num) = current_number {
                if has_current_num_adjacent_symbol {
                    total += num;
                }

                current_number = None;
                has_current_num_adjacent_symbol = false;
            }
        }

        if let Some(num) = current_number {
            if has_current_num_adjacent_symbol {
                total += num;
            }

            current_number = None;
            has_current_num_adjacent_symbol = false;
        }
    }

    total
}

#[aoc(day3, part2)]
pub fn part2(input: &EngineSchematic) -> u32 {
    let mut total = 0_u32;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            let adj = find_adjacent_numbers(input, x, y);
            if adj.len() != 2 {
                continue;
            }

            total += adj.iter().product::<u32>();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day3::input_schematic;

    #[test]
    fn test_part1() {
        let schematic = input_schematic("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(super::part1(&schematic), 4361);
    }

    #[test]
    fn test_part2() {

        let schematic = input_schematic("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(super::part2(&schematic), 467835);
    }

}