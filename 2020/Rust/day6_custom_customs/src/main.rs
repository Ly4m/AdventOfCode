use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let lines = fs::read_to_string("src/declaration_form.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let part_1 = solve_part_1(&lines);
    println!("{}", part_1);

    let part_2 = solve_part_2(&lines);
    println!("{}", part_2);
}

fn solve_part_1(lines: &Vec<String>) -> usize {
    lines
        .split(|l| l.len() == 0)
        .map(|l| count_distinct_answers(l))
        .sum()
}

fn count_distinct_answers(group: &[String]) -> usize {
    let all_group_answers: Vec<char> = group.iter().flat_map(|c| c.chars()).collect();
    let counts:HashSet<char> = HashSet::from_iter(all_group_answers.iter().map(|x| *x));
    counts.len()
}

fn solve_part_2(lines: &Vec<String>) -> usize {
    lines
        .split(|l| l.len() == 0)
        .map(|l| count_common_answers(l))
        .sum()
}

fn count_common_answers(group: &[String]) -> usize {
    let mut group_answers: HashSet<char> = HashSet::from_iter(group.first().unwrap().chars());

    for answer in group {
        let current_answer_chars = HashSet::from_iter(answer.chars());
        let intersection = group_answers.intersection(&current_answer_chars);
        group_answers = HashSet::from_iter(intersection.into_iter().map(|x| *x));
    }
    group_answers.len()
}


#[cfg(test)]
mod tests {
    use crate::count_common_answers;

    #[test]
    fn simple_test() {
        let group = [String::from("abc"), String::from("b")];
        assert_eq!(1, count_common_answers(&group))
    }
}

