const DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let contents = include_str!("input.txt");

    println!("first star: {}", part1(contents));
    println!("second star: {}", part2(contents));
}

fn part1(contents: &str) -> u32 {
    contents
        .lines()
        .map(|line| {
            let d1 = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let d2 = line
                .chars()
                .rfind(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            d1 * 10 + d2
        })
        .sum()
}

fn part2(contents: &str) -> u32 {
    let mut total = 0;

    for line in contents.lines() {
        let mut d1 = 0;
        'search: for i in 0..line.len() {
            for (digit_str, digit) in DIGITS {
                if line[0..i].ends_with(digit_str) {
                    d1 = *digit;
                    break 'search;
                }
            }
            if let Some(c) = line.chars().nth(i) {
                if c.is_digit(10) {
                    d1 = c.to_digit(10).unwrap();
                    break;
                }
            }
        }

        let mut d2 = 0;
        'search: for i in (0..line.len()).rev() {
            for (digit_str, digit) in DIGITS {
                if line[i..line.len()].starts_with(digit_str) {
                    d2 = *digit;
                    break 'search;
                }
            }
            if let Some(c) = line.chars().nth(i) {
                if c.is_digit(10) {
                    d2 = c.to_digit(10).unwrap();
                    break;
                }
            }
        }

        total += d1 * 10 + d2;
    }

    total
}
