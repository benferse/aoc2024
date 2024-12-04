//! Day 2 - Red-Nosed Reports

use itertools::*;
use std::cmp::Ordering;

pub fn is_safe(report: &[u64]) -> bool {
    let mut saw_increase = false;
    let mut saw_decrease = false;

    for pair in report.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if diff == 0 || diff > 3 {
            return false;
        }

        match pair[0].cmp(&pair[1]) {
            Ordering::Less => saw_increase = true,
            Ordering::Greater => saw_decrease = true,
            Ordering::Equal => return false,
        }
    }

    saw_increase ^ saw_decrease
}

pub fn is_safe_with_dampening(report: &[u64]) -> bool {
    for n in 0..report.len() {
        // Filter out the nth level
        let v = report
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if i != n { Some(*e) } else { None })
            .collect_vec();

        if is_safe(&v) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = include_str!("../input/day2-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("../input/day2-real.txt");

    #[test_case(SAMPLE_INPUT => 2; "with sample data")]
    #[test_case(PERSONAL_INPUT => 314; "with real data")]
    fn problem1(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|word| word.parse::<u64>().ok())
                    .collect_vec()
            })
            .filter(|report| is_safe(report))
            .count()
    }

    #[test_case(SAMPLE_INPUT => 4; "with sample data")]
    #[test_case(PERSONAL_INPUT => 373; "with real data")]
    fn problem2(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|word| word.parse::<u64>().ok())
                    .collect_vec()
            })
            .filter(|report| is_safe(report) || is_safe_with_dampening(report))
            .count()
    }
}
