fn main() {
    let input: &str = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    // Split each string in two halves, find the common char and assign priority
    let mut score: i32 = 0;
    for line in input.lines() {
        let (beginning, end) = line.split_at(line.len() / 2);
        for c in beginning.chars() {
            if end.contains(c) {
                score += get_priority(c);
                break;
            }
        }
    }
    score
}

fn part2(input: &str) -> i32 {
    // Iterate three lines at a time, find the common char and assign priority
    let mut score: i32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for chunk in lines.chunks(3) {
        if let [first, second, third] = chunk {
            for c in first.chars() {
                if second.contains(c) && third.contains(c) {
                    score += get_priority(c);
                    break;
                }
            }
        }
    }
    score
}

fn get_priority(c: char) -> i32 {
    // Translate to badge priority
    // a-z = 1-26, A-Z = 27-52
    if c.is_lowercase() {
        c as i32 - 96 // ASCII a == 97
    } else {
        c as i32 - 65 + 27 // ASCII A == 65
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(super::part1(TEST_INPUT), 157)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT), 70);
    }
}
