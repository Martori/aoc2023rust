use std::str::FromStr;
use crate::helpers::read_input;

pub fn solve() {
    let input = read_input("Day01");
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part1(input: &Vec<String>) -> u32 {
    let mut total = 0u32;
    for line in input {
        let mut first: Option<char> = None;
        let mut current: Option<char> = None;
        for c in line.chars() {
            if char::is_digit(c, 10) && first.is_none() {
                first = Some(c);
                current = Some(c);
            } else if char::is_digit(c, 10) {
                current = Some(c)
            }
        }
        let f = first.expect("missing first digit");
        let l = current.expect("missing last digit");
        let num = u32::from_str(&*format!("{f}{l}")).expect("Invalid formatted numbers");
        total = total + num
    }
    total
}

const NUMBERS: [(&str, &str); 9] = [("one", "o1e"), ("two", "t2o"), ("three", "t3e"), ("four", "f4r"), ("fve", "f5e"), ("six", "s6x"), ("seven", "s7n"), ("eight", "e8t"), ("nine", "n9e")];

fn part2(input: &Vec<String>) -> u32 {
    part1(&input.iter().map(|l| {
        let mut n = l.to_string();
        for (from, to) in NUMBERS {
            n = n.replace(from, to);
        }
        n
    }
    ).collect())
}

#[cfg(test)]
mod tests {
    use crate::days::day01::part1;
    use crate::days::day01::part2;
    use crate::helpers::read_input;

    #[test]
    fn part1test() {
        let result = part1(&read_input("Day01Sample1"));
        assert_eq!(result, 142);
    }

    #[test]
    fn part2test() {
        let result = part2(&read_input("Day01Sample2"));
        assert_eq!(result, 281);
    }
}