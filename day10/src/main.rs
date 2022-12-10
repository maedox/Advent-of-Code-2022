use crate::Operation::*;

const INPUT: &str = include_str!("../input.txt");

enum Operation {
    Noop,
    Add,
}

fn parse_input(input: &str) -> (Operation, i64) {
    if input.starts_with('n') {
        (Noop, 0)
    } else {
        let num = input.split_whitespace().last().unwrap().parse().unwrap();
        (Add, num)
    }
}

fn count_cycles(input: &str) -> i64 {
    let mut x = 1;
    let mut signal_strengths: Vec<i64> = vec![];
    let mut lines = input.lines();

    let mut c: usize = 1;

    loop {
        check_signal(c, &mut signal_strengths, x);
        if let Some(line) = lines.next() {
            match parse_input(line) {
                (Noop, 0) => {}
                (Add, num) => {
                    c += 1;
                    check_signal(c, &mut signal_strengths, x);
                    x += num;
                }
                _ => (),
            }
            c += 1;
        } else {
            check_signal(c, &mut signal_strengths, x);
            break;
        }
    }
    dbg!(signal_strengths.iter().sum())
}

fn check_signal(c: usize, signal_strengths: &mut Vec<i64>, x: i64) {
    if [20, 60, 100, 140, 180, 220].contains(&c) {
        signal_strengths.push(c as i64 * x)
    }
}

fn main() {
    println!("Part 1: {}", count_cycles(INPUT));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../sample-input.txt");

    #[test]
    fn part_1() {
        assert_eq!(count_cycles(TEST_INPUT), 13140);
        assert_eq!(count_cycles(INPUT), 14040);
    }
}
