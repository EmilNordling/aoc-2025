type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> i32 {
    input.iter().map(|l| get_joltage_from_bank(l)).sum::<i32>()
}

pub fn part2(input: &Input) -> i64 {
    input
        .iter()
        .map(|l| get_twelve_largest_joltages_from_bank(l))
        .sum::<i64>()
}

fn get_joltage_from_bank(bank: &str) -> i32 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut max_joltage = 0;
    let mut max_first_digit = 0;

    for &digit in &digits {
        if max_first_digit > 0 {
            let joltage = max_first_digit * 10 + digit;
            max_joltage = max_joltage.max(joltage);
        }

        max_first_digit = max_first_digit.max(digit);
    }

    max_joltage as i32
}

fn get_twelve_largest_joltages_from_bank(bank: &str) -> i64 {
    let digits_to_remove = bank.len() - 12;
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut result = Vec::new();

    for (i, &d) in digits.iter().enumerate() {
        while !result.is_empty()
            && result.len() + digits_to_remove > i
            && result.last().unwrap() < &d
        {
            result.pop();
        }

        result.push(d);
    }

    result
        .into_iter()
        .take(12)
        .fold(0i64, |acc, d| acc * 10 + d as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_get_joltage_from_bank() {
        assert_eq!(get_joltage_from_bank("123"), 23);
        assert_eq!(get_joltage_from_bank("987654321111111"), 98);
        assert_eq!(get_joltage_from_bank("811111111111119"), 89);
        assert_eq!(get_joltage_from_bank("234234234234278"), 78);
        assert_eq!(get_joltage_from_bank("818181911112111"), 92);
    }

    #[test]
    fn test_part1() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_get_twelve_largest_joltages_from_bank() {
        assert_eq!(
            get_twelve_largest_joltages_from_bank("987654321111111"),
            987654321111
        );
        assert_eq!(
            get_twelve_largest_joltages_from_bank("811111111111119"),
            811111111119
        );
        assert_eq!(
            get_twelve_largest_joltages_from_bank("234234234234278"),
            434234234278
        );
        assert_eq!(
            get_twelve_largest_joltages_from_bank("818181911112111"),
            888911112111
        );
    }

    #[test]
    fn test_part2() {
        let input = parse(&TEST_INPUT);
        assert_eq!(part2(&input), 3121910778619);
    }
}
