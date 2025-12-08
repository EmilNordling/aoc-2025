type Input<'a> = Vec<Vec<char>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut grid = input.clone();
    let rows = grid.len() - 1;
    let cols = grid[0].len() - 1;

    let mut tachyon_beams = Vec::<usize>::new();
    for col in 0..cols {
        if grid[0][col] == 'S' {
            tachyon_beams.push(col);
            break;
        }
    }

    let mut splitted = 0;
    let mut depth = 1;

    loop {
        depth += 1;

        if depth > rows {
            break;
        }

        let mut new_tachyon_beams = Vec::<usize>::new();
        for &beam_col in &tachyon_beams {
            match grid[depth][beam_col] {
                '.' => {
                    grid[depth][beam_col] = '|';
                    new_tachyon_beams.push(beam_col);
                }
                '^' => {
                    grid[depth][beam_col - 1] = '|';
                    grid[depth][beam_col + 1] = '|';
                    new_tachyon_beams.push(beam_col - 1);
                    new_tachyon_beams.push(beam_col + 1);
                    splitted += 1;
                }
                _ => {}
            }
        }

        tachyon_beams = new_tachyon_beams;
    }

    splitted
}

pub fn part2<'a>(_input: &Input) -> &'a str {
    "0"
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let input = parse(TEST_INPUT);
        assert_eq!(part1(&input), 21);
    }
}
