type Input<'a> = Vec<Vec<&'a str>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    let operations = input[input.len() - 1].clone();

    for x_axis in 0..operations.len() {
        let mut col_sum = input[0][x_axis].parse::<i64>().unwrap();

        for y_axis in 1..input.len() - 1 {
            let num = input[y_axis][x_axis].parse::<i64>().unwrap();

            match operations[x_axis] {
                "*" => col_sum *= num,
                "+" => col_sum += num,
                _ => panic!("rip"),
            }
        }

        sum += col_sum;
    }

    sum
}

pub fn part2(_input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let input = parse(TEST_INPUT);
        assert_eq!(part1(&input), 4277556);
    }
}
