//! Day1 - Historian Hysteria

pub fn parse_lists(content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_readings = vec![];
    let mut right_readings = vec![];

    content.lines().for_each(|line| {
        let vals = line.split_whitespace();
        let mut nums = vals.map(|s| s.parse::<i32>().unwrap());
        left_readings.push(nums.next().unwrap());
        right_readings.push(nums.next().unwrap());
    });

    left_readings.sort();
    right_readings.sort();

    (left_readings, right_readings)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    const SAMPLE_DATA: &str = include_str!("../input/day1-sample.txt");
    const PERSONAL_DATA: &str = include_str!("../input/day1-real.txt");

    #[test_case(SAMPLE_DATA => 11; "with sample data")]
    #[test_case(PERSONAL_DATA => 2769675; "with real data")]
    fn problem1(input: &str) -> i32 {
        let (left, right) = parse_lists(input);
        std::iter::zip(left, right)
            .map(|(left, right)| (left - right).abs())
            .sum()
    }

    #[test_case(SAMPLE_DATA => 31; "with sample data")]
    #[test_case(PERSONAL_DATA => 24643097; "with real data")]
    fn problem2(input: &str) -> i32 {
        let (left, right) = parse_lists(input);

        left.iter()
            .map(|e| {
                let c = right.iter().filter(|&n| n == e).count() as i32;
                e * c
            })
            .sum()
    }
}
