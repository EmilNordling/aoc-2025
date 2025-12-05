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

pub fn part1(input: &Input) -> usize {
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

pub fn part2(input: &Input) -> usize {
    let mut sorted_ranges = input.0.clone();
    sorted_ranges.sort_unstable_by_key(|(start, _)| *start);

    sorted_ranges
        .into_iter()
        .fold(Vec::new(), |mut merged: Vec<(i64, i64)>, (start, end)| {
            match merged.last_mut() {
                Some(last) if start <= last.1 + 1 => {
                    last.1 = last.1.max(end);
                }
                _ => merged.push((start, end)),
            }
            merged
        })
        .into_iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum()
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

    #[test]
    fn test_part2() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part2(&input), 14);
    }
}
