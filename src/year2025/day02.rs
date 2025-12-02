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
