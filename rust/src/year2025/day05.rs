type Input = (Vec<(i64, i64)>, Vec<i64>);

pub fn parse(input: &str) -> Input {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let ranges = groups[0]
        .trim()
        .split("\n")
        .map(|s| {
            let mut parts = s.split("-");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let numbers = groups[1]
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    (ranges, numbers)
}

pub fn part1(input: &Input) -> i64 {
    let (ranges, numbers) = input;
    let mut result = 0;

    for number in numbers {
        for (start, end) in ranges {
            if start <= number && number <= end {
                result += 1;
                break;
            }
        }
    }

    result
}

pub fn part2<'a>(_input: &'a Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part1(&input), 3);
    }
}
