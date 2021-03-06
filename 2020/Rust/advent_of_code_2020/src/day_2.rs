use std::fs;

pub fn solve_part_1() -> usize {
    fs::read_to_string("inputs/day_2.in")
        .unwrap()
        .lines()
        .filter(|s| is_password_compliant(s))
        .count()
}

pub fn solve_part_2() -> usize {
    fs::read_to_string("inputs/day_2.in")
        .unwrap()
        .lines()
        .filter(|s| is_password_compliant_with_otcp(s))
        .count()
}

fn is_password_compliant(line: &str) -> bool {
    let splits: Vec<String> = line.split_whitespace()
        .map(String::from)
        .collect();

    let mut p1 = splits.get(0).unwrap().split('-');
    let min = p1.next().unwrap().parse::<usize>().unwrap();
    let max = p1.next().unwrap().parse::<usize>().unwrap();

    let char = splits.get(1).unwrap().chars().next().unwrap();
    let pwd = splits.get(2).unwrap();

    let count = pwd.chars().filter(|c| *c == char).count();

    min <= count && max >= count
}

fn is_password_compliant_with_otcp(line: &str) -> bool {
    let splits: Vec<String> = line.split_whitespace()
        .map(String::from)
        .collect();

    let mut positions = splits.get(0).unwrap().split('-');
    let p1 = positions.next().unwrap().parse::<usize>().unwrap();
    let p2 = positions.next().unwrap().parse::<usize>().unwrap();

    let char = splits.get(1).unwrap().chars().next().unwrap();
    let pwd = splits.get(2).unwrap();

    let matches_p1 = *pwd.chars().collect::<Vec<char>>().get(p1 - 1).unwrap() == char;
    let matches_p2 = *pwd.chars().collect::<Vec<char>>().get(p2 - 1).unwrap() == char;

    matches_p1 ^ matches_p2
}

#[cfg(test)]
mod tests {
    use crate::day_2::{is_password_compliant, is_password_compliant_with_otcp};

    #[test]
    fn simple_test() {
        assert_eq!(true, is_password_compliant("1-3 a: abcde"))
    }

    #[test]
    fn simple_test2() {
        assert_eq!(false, is_password_compliant("1-3 b: cdefg"))
    }

    #[test]
    fn simple_test3() {
        assert_eq!(true, is_password_compliant("2-9 c: ccccccccc"))
    }

    #[test]
    fn simple_test_v2_1() {
        assert_eq!(true, is_password_compliant_with_otcp("1-3 a: abcde"))
    }

    #[test]
    fn simple_test_v2_2() {
        assert_eq!(false, is_password_compliant_with_otcp("1-3 b: cdefg"))
    }

    #[test]
    fn simple_test_v2_3() {
        assert_eq!(false, is_password_compliant_with_otcp("2-9 c: ccccccccc"))
    }
}
