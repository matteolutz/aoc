#[aoc_generator(day8)]
pub fn input_trees(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}

pub fn is_visible(trees: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == 0 || x == trees[0].len() - 1 || y == 0 || y == trees.len() - 1 {
        return true;
    }

    let tree_val = trees[y][x];

    let mut visible_top = true;
    for y_it in 0..y {
        if trees[y_it][x] >= tree_val {
            visible_top = false;
            break;
        }
    }

    let mut visible_bottom = true;
    for y_it in y+1..trees.len() {
        if trees[y_it][x] >= tree_val {
            visible_bottom = false;
            break;
        }
    }

    let mut visible_left = true;
    for x_it in 0..x {
        if trees[y][x_it] >= tree_val {
            visible_left = false;
            break;
        }
    }

    let mut visible_right = true;
    for x_it in x+1..trees[0].len() {
        if trees[y][x_it] >= tree_val {
            visible_right = false;
            break;
        }
    }

    visible_top || visible_left || visible_right || visible_bottom
}

pub fn get_scenic_score(trees: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let tree_val = trees[y][x];

    let mut score_top = 0;
    for y_it in (0..y).rev() {
        score_top += 1;
        if trees[y_it][x] >= tree_val {
            break;
        }
    }

    let mut score_bottom = 0;
    for y_it in y+1..trees.len() {
        score_bottom += 1;
        if trees[y_it][x] >= tree_val {
            break;
        }
    }

    let mut score_left = 0;
    for x_it in (0..x).rev() {
        score_left += 1;
        if trees[y][x_it] >= tree_val {
            break;
        }
    }

    let mut score_right = 0;
    for x_it in x+1..trees[0].len() {
        score_right += 1;
        if trees[y][x_it] >= tree_val {
            break;
        }
    }

    score_top * score_bottom * score_left * score_right
}

#[aoc(day8, part2)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| get_scenic_score(input, x, y))
        })
        .max().unwrap()
}