use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", decode(INPUT, 4));
    println!("Part 2: {}", decode(INPUT, 14));
}

fn decode(input: &str, marker_size: usize) -> usize {
    // Return the string index after marker_size consecutive unique characters
    for i in 0..input.len() - marker_size + 1 {
        let window = &input[i..i + marker_size];
        let set: HashSet<char> = window.chars().collect();
        if set.len() == marker_size {
            return i + marker_size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part1() {
        assert_eq!(decode("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(decode("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(decode("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(decode("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(decode("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
        assert_eq!(decode(INPUT, 4), 1287);
    }

    #[test]
    fn part2() {
        assert_eq!(decode("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(decode("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(decode("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(decode("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(decode("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
        assert_eq!(decode(INPUT, 14), 3716);
    }
}
