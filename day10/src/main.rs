use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use std::{error::Error, iter, time::Instant};
use Operation::*;

const INPUT: &str = include_str!("../input.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Noop,
    Add(i32),
}

fn parse_operation(input: &str) -> IResult<Operation> {
    alt((tag("noop").value(Noop), tag("addx ").precedes(i32).map(Add)))(input)
}

type Parsed = Vec<Operation>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_operation)(data)
}

fn iter_register(data: &Parsed) -> impl Iterator<Item = i32> + '_ {
    data.iter()
        .scan(1, |register, op| {
            Some(repeat_n(
                *register, // dereference before mutating
                match op {
                    Noop => 1,
                    Add(x) => {
                        *register += x;
                        2
                    }
                },
            ))
        })
        .flatten()
}

fn check_signal_strength(data: &Parsed) -> i32 {
    iter_register(data)
        .zip(1..)
        .filter(|(_, cycle)| [20, 60, 100, 140, 180, 220].contains(cycle))
        .map(|(reg_x, cycle)| reg_x * cycle)
        .sum()
}

fn draw_pixels(data: &Parsed) -> String {
    iter_register(data)
        .chunks(40)
        .into_iter()
        .flat_map(|row| {
            row.zip(0..)
                .map(|(x, pos)| if x.abs_diff(pos) <= 1 { '#' } else { '.' })
                .chain(iter::once('\n'))
        })
        .collect()
}

fn main() -> OutResult {
    let duration = Instant::now();
    let parsed = parse(INPUT)?.1;
    println!("Part 1: {}", check_signal_strength(&parsed));
    println!("Part 2:\n{}", draw_pixels(&parsed));
    println!("Finished in {:?}", duration.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const TEST_INPUT: &str = include_str!("../sample-input.txt");

    #[test]
    fn test_1() -> OutResult {
        assert_eq!(check_signal_strength(&parse(TEST_INPUT)?.1), 13140);
        assert_eq!(check_signal_strength(&parse(INPUT)?.1), 14040);
        Ok(())
    }

    #[test]
    fn part_2() -> OutResult {
        assert_eq!(
            draw_pixels(&parse(TEST_INPUT)?.1),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
        Ok(())
    }
}
