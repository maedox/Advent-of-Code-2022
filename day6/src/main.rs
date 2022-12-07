use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", decode(INPUT, 4));
    println!("Part 2: {}", decode(INPUT, 14));
}

// Return the string index after marker_size consecutive unique characters
fn decode(input: &str, marker_size: usize) -> usize {
    input
        .graphemes(true)
        .collect::<Vec<&str>>()
        .windows(marker_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == marker_size)
        .map(|pos| pos + marker_size)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part1() {
        assert_eq!(decode("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(decode("åjqjpqågbljsphdztnvjfqwrcgsålb", 4), 7);
        assert_eq!(decode("🦀jqjpq🦀gbljsphdztnvjfqwrcgs🦀lb", 4), 7);
        assert_eq!(decode("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(decode("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(decode("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(decode("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
        assert_eq!(decode(INPUT, 4), 1287);
    }

    #[test]
    fn part2() {
        assert_eq!(decode("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(decode("åjqjpqågbljsphdztnvjfqwrcgsålb", 14), 19);
        assert_eq!(decode("🦀jqjpq🦀gbljsphdztnvjfqwrcgs🦀lb", 14), 19);
        assert_eq!(decode("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(decode("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(decode("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(decode("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
        assert_eq!(decode(INPUT, 14), 3716);
    }
}
