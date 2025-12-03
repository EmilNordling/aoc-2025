type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> i32 {
    input.iter().map(|l| get_joltage_from_bank(l)).sum::<i32>()
}

pub fn part2(_input: &Input) -> i32 {
    0
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
}
