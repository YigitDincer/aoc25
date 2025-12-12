#[derive(Debug, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

#[derive(Debug, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    val: i32,
}

fn parse(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (direction_str, val_str) = line.split_at(1);
            let direction: Direction = match direction_str {
                "L" => Direction::L,
                "R" => Direction::R,
                _ => panic!("unexpected char: {}", direction_str),
            };
            let val = val_str.parse().unwrap();
            Rotation { direction, val }
        })
        .collect()
}

fn rotate_and_count_zeroes(current_pos: i32, rotation: &Rotation) -> (i32, i32) {
    let mut new_pos = current_pos;
    let Rotation { direction, val } = &rotation;

    let mut ctr = 0;

    match direction {
        Direction::L => {
            new_pos -= val;

            while new_pos < 0 {
                ctr += 1;
                new_pos += 100
            }

            if current_pos == 0 {
                ctr -= 1;
            }

            if new_pos == 0 {
                ctr += 1;
            }
        }
        Direction::R => {
            new_pos += val;
            while new_pos >= 100 {
                ctr += 1;
                new_pos -= 100;
            }
        }
    }

    (new_pos, ctr)
}

pub fn solve(input: &str) {
    println!(
        "{}",
        parse(input)
            .iter()
            .scan(50, |acc, rotation| {
                *acc = rotate_and_count_zeroes(*acc, rotation).0;
                Some(*acc)
            })
            .filter(|&current_pos| current_pos == 0)
            .count()
    );

    println!(
        "{}",
        parse(input)
            .iter()
            .scan(50, |acc, rotation| {
                let (left, right) = rotate_and_count_zeroes(*acc, rotation);
                *acc = left;
                Some(right)
            })
            .sum::<i32>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SHORT: &str = "L68
R30";

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const EXAMPLE_CUSTOM: &str = "L50
L1
R1";

    fn example_rotations() -> Vec<Rotation> {
        vec![
            Rotation {
                direction: Direction::L,
                val: 68,
            },
            Rotation {
                direction: Direction::R,
                val: 30,
            },
        ]
    }

    #[test]
    fn parse_example_short() {
        assert_eq!(parse(EXAMPLE_SHORT), example_rotations());
    }

    #[test]
    fn rotate_short() {
        assert_eq!(
            example_rotations()
                .iter()
                .fold(0, |acc, rotation| rotate_and_count_zeroes(acc, rotation).0),
            62
        );
    }

    #[test]
    fn rotate_long() {
        assert_eq!(
            parse(EXAMPLE)
                .iter()
                .fold(50, |acc, rotation| rotate_and_count_zeroes(acc, rotation).0),
            32
        );
    }

    #[test]
    fn count_zeroes_after_rotate() {
        assert_eq!(
            parse(EXAMPLE)
                .iter()
                .scan(50, |acc, rotation| {
                    *acc = rotate_and_count_zeroes(*acc, rotation).0;
                    Some(*acc)
                })
                .filter(|&current_pos| current_pos == 0)
                .count(),
            3
        );
    }

    #[test]
    fn count_zeroes_during_rotate() {
        assert_eq!(
            parse(EXAMPLE)
                .iter()
                .scan(50, |acc, rotation| {
                    let (left, right) = rotate_and_count_zeroes(*acc, rotation);
                    *acc = left;
                    Some(right)
                })
                .sum::<i32>(),
            6
        );
    }

    #[test]
    fn count_zeroes_during_rotate_custom() {
        assert_eq!(
            parse(EXAMPLE_CUSTOM)
                .iter()
                .scan(50, |acc, rotation| {
                    *acc = rotate_and_count_zeroes(*acc, rotation).1;
                    Some(*acc)
                })
                .sum::<i32>(),
            2
        );
    }
}
