type Input = Vec<Vec<i64>>;

pub fn parse(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|pair| {
            pair.split('-')
                .filter_map(|n| n.parse::<i64>().ok())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    input
        .iter()
        .flat_map(|range| {
            if range.len() == 2 {
                (range[0]..=range[1])
                    .filter(|&num| {
                        let s = num.to_string();
                        is_invalid_id(&s)
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .sum()
}

pub fn part2(_input: &Input) -> i64 {
    0
}

fn is_invalid_id(id: &str) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    &id[..mid] == &id[mid..]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_invalid_id_part1() {
        assert!(is_invalid_id("11"));
        assert!(is_invalid_id("22"));
        assert!(is_invalid_id("1010"));
        assert!(is_invalid_id("1188511885"));
        assert!(is_invalid_id("222222"));
        assert!(is_invalid_id("446446"));
        assert!(is_invalid_id("38593859"));
    }

    #[test]
    fn test_part1() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part1(&input), 1227775554);
    }
}
