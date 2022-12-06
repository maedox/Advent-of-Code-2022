#![feature(get_many_mut)]

// Heavily inspired by https://fasterthanli.me/series/advent-of-code-2022/part-5

use crate::CraneModel::*;

use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {i}: {pile:?}")?;
        }
        Ok(())
    }
}
impl Piles {
    fn apply(&mut self, ins: Instruction, crane_model: &CraneModel) {
        let [src, dst] = self
            .0
            .get_many_mut([ins.src, ins.dst])
            .expect("out of bounds / overlapping src/dst stacks");

        match crane_model {
            CrateMover9000 => dst.extend(src.drain((src.len() - ins.quantity)..).rev()),
            CrateMover9001 => dst.extend(src.drain((src.len() - ins.quantity)..)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // `drop` takes a value and returns nothing, which is perfect for our case
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u32, |n| n as _)(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

// Parse lines like "move # from # to #"
fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

// Transpose rows to columns
fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            let mut v = Vec::with_capacity(256); // just to be on the safe side
            v.extend(iters.iter_mut().rev().filter_map(|n| n.next().unwrap()));
            v
        })
        .collect()
}

fn process_input(input: &str, crane_model: CraneModel) -> String {
    let mut lines = input.lines();

    let crate_lines: Vec<_> = lines
        .by_ref()
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = Piles(transpose_rev(crate_lines));

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        piles.apply(ins, &crane_model);
    }

    // return the top crate from each pile
    piles
        .0
        .iter()
        .map(|pile| pile.last().unwrap().0)
        .collect::<String>()
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", process_input(INPUT, CrateMover9000));
    println!("Part 2: {}", process_input(INPUT, CrateMover9001));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn part_1() {
        // grab one crate at a time
        assert_eq!(process_input(TEST_INPUT, CrateMover9000), "CMZ");
        assert_eq!(process_input(INPUT, CrateMover9000), "VQZNJMWTR");
    }
    #[test]
    fn part_2() {
        // grab many crates simultaneously
        assert_eq!(process_input(TEST_INPUT, CrateMover9001), "MCD");
        assert_eq!(process_input(INPUT, CrateMover9001), "NLCDCLVMQ");
    }
}
