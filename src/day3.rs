#[cfg(test)]
mod test {
    use regex::Regex;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = include_str!("../input/day3-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("../input/day3-real.txt");

    #[test_case(SAMPLE_INPUT => 161; "with sample data")]
    #[test_case(PERSONAL_INPUT => 187194524; "with real data")]
    fn problem1(input: &str) -> u64 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Should be a valid regex");
        let mut res = 0;
        for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
            res += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
        }
        res
    }
}
