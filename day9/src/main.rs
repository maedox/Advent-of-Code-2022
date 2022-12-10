use crate::Direction::*;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", get_points(INPUT));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn step_one(mut self, dir: Direction) -> Self {
        match dir {
            Up => self.y += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
        self
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_instruction(line: &str) -> (Direction, u8) {
    let mut parts = line.split_whitespace();
    let direction = match &parts.next() {
        Some("U") => Up,
        Some("D") => Down,
        Some("L") => Left,
        Some("R") => Right,
        _ => panic!("Invalid direction."),
    };
    let distance = parts.next().unwrap().parse().unwrap();
    (direction, distance)
}

fn get_points(input: &str) -> usize {
    let mut head = Point { x: 1, y: 1 };
    let mut prev_head = Point { x: 1, y: 1 };
    let mut tail = Point { x: 1, y: 1 };
    let mut seen_positions = HashSet::new();

    seen_positions.insert(tail);

    for line in input.lines() {
        let (direction, distance) = parse_instruction(line);
        for _ in 0..distance {
            head = head.step_one(direction);
            if tail_should_move(head, tail) {
                tail = prev_head;
                seen_positions.insert(tail);
            };
            prev_head = head;
        }
    }
    seen_positions.len()
}

// If distance between head and tail is > 1, it must move to stay at distance 1.
fn tail_should_move(head: Point, tail: Point) -> bool {
    (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        assert_eq!(get_points(TEST_INPUT), 13);
        assert_eq!(get_points(INPUT), 6271);
    }
}
