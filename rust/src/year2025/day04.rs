type Input = Vec<Vec<char>>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1<'a>(input: &'a Input) -> i32 {
    let mut count = 0;

    for (i, x) in input.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            let value = get_adjacent_rolls(&input, i as i32, j as i32);

            if value != 0 {
                count += value;
                continue;
            }
        }
    }

    count
}

static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_adjacent_rolls(matrix: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    let height = matrix.len();
    let width = matrix[0].len();

    let mut count = 0;

    let origin = matrix[row as usize][col as usize];
    if origin != '@' {
        return 0;
    }

    for (dx, dy) in DIRECTIONS {
        let new_row = row as i32 + dx;
        let new_col = col as i32 + dy;

        if new_row >= 0 && new_row < height as i32 && new_col >= 0 && new_col < width as i32 {
            let ch = matrix[new_row as usize][new_col as usize];
            if ch == '@' {
                count += 1;
            }

            if count >= 4 {
                return 0;
            }
        }
    }

    1
}

pub fn part2<'a>(_input: &'a Input) -> &'a str {
    "0"
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }
}
