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

pub fn flip_number(n: u32) -> u32 {
    let mut flipped = 0_u32;
    let mut n = n;
    while n > 0 {
        flipped *= 10;
        flipped += n % 10;
        n /= 10;
    }

    flipped
}

pub fn has_adjacent_symbol(schematic: &EngineSchematic, x: usize, y: usize) -> (bool, usize, usize) {
    // left
    if x > 0 && is_symbol(&schematic[y][x - 1]) {
        return (true, x - 1, y);
    }

    // right
    if x < schematic[y].len() - 1 && is_symbol(&schematic[y][x + 1]) {
        return (true, x + 1, y);
    }

    // above
    if y > 0 && is_symbol(&schematic[y - 1][x]) {
        return (true, x, y - 1);
    }

    // below
    if y < schematic.len() - 1 && is_symbol(&schematic[y + 1][x]) {
        return (true, x, y + 1);
    }

    // diagonally left above
    if x > 0 && y > 0 && is_symbol(&schematic[y - 1][x - 1]) {
        return (true, x - 1, y - 1);
    }

    // diagonally right above
    if x < schematic[y].len() - 1 && y > 0 && is_symbol(&schematic[y - 1][x + 1]) {
        return (true, x + 1, y - 1);
    }

    // diagonally left below
    if x > 0 && y < schematic.len() - 1 && is_symbol(&schematic[y + 1][x - 1]) {
        return (true, x - 1, y + 1);
    }

    // diagonally right below
    if x < schematic[y].len() - 1 && y < schematic.len() - 1 && is_symbol(&schematic[y + 1][x + 1]) {
        return (true, x + 1, y + 1);
    }

    (false, 0, 0)
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

#[cfg(test)]
mod tests {
    use crate::day3::input_schematic;

    #[test]
    fn test_part_1() {
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
}