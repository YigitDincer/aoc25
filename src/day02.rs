use itertools::Itertools;

pub fn solve(input: &str) {
    println!(
        "{}",
        parse(input)
            .into_iter()
            .map(|range| sum_invalid_nums(range, Part::Part1))
            .sum::<u64>()
    );

    println!(
        "{}",
        parse(input)
            .into_iter()
            .map(|range| sum_invalid_nums(range, Part::Part2))
            .sum::<u64>()
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Part {
    Part1,
    Part2,
}

fn is_valid(number: u64, part: Part) -> bool {
    let digit_count = number.ilog10() as usize + 1;

    match part {
        Part::Part1 => {
            if digit_count % 2 == 0 {
                let middle_divider = 10u64.pow((digit_count / 2).try_into().unwrap());
                return (number / middle_divider) != (number % middle_divider);
            }

            true
        }
        Part::Part2 => {
            let number_str = number.to_string();
            !(1..=digit_count / 2).any(|seq| number_str.as_bytes().chunks(seq).all_equal())
        }
    }
}

fn parse(input: &str) -> Vec<std::ops::Range<u64>> {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| std::ops::Range {
            start: start.parse::<u64>().unwrap(),
            end: end.parse::<u64>().unwrap() + 1,
        })
        .collect()
}

fn sum_invalid_nums(range: std::ops::Range<u64>, part: Part) -> u64 {
    range.filter(|&number| !is_valid(number, part)).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_valid() {
        assert_eq!(
            parse(&EXAMPLE)
                .into_iter()
                .map(|range| sum_invalid_nums(range, Part::Part1))
                .sum::<u64>(),
            1227775554
        );
    }

    #[test]
    fn test_is_valid_extended() {
        assert_eq!(
            parse(&EXAMPLE)
                .into_iter()
                .map(|range| sum_invalid_nums(range, Part::Part2))
                .sum::<u64>(),
            4174379265
        );
    }
}
