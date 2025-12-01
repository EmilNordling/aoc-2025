type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let line = l.trim();
            if line.is_empty() {
                return None;
            }

            let number: i32 = line[1..].parse().ok()?;

            match line.as_bytes()[0] {
                b'R' => Some(number),
                b'L' => Some(-number),
                _ => None,
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut point = 50;
    let mut password = 0;

    for &step in input {
        point = (point + step).rem_euclid(100);
        if point == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &Input) -> i32 {
    let mut point = 50;
    let mut password = 0;
    let rhs = 100;

    for &step in input {
        if step >= 0 {
            password += (point + step) / rhs;
        } else {
            let reversed = (rhs - point) % rhs;
            password += (reversed - step) / rhs;
        }

        point = (point + step).rem_euclid(rhs);
    }

    password
}
