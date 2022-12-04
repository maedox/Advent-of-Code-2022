use color_eyre::eyre::{eyre, Report};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {
    fn contains(self, value: &u8) -> bool {
        let range = self.start..=self.end;
        range.contains(value)
    }
}

fn find_parts(line: &str) -> Result<(Range, Range), Report> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<first_start>\d+)-(?P<first_end>\d+),(?P<last_start>\d+)-(?P<last_end>\d+)"
        )
        .unwrap();
    }
    if let Some(groups) = RE.captures(line) {
        Ok((
            Range {
                start: groups["first_start"].parse::<u8>()?,
                end: groups["first_end"].parse::<u8>()?,
            },
            Range {
                start: groups["last_start"].parse::<u8>()?,
                end: groups["last_end"].parse::<u8>()?,
            },
        ))
    } else {
        Err(eyre!("Failed to find two groups of two u8 in line.",))
    }
}

fn score(input: &str) -> Result<(u32, u32), Report> {
    let mut p1_score = 0;
    let mut p2_score = 0;
    for line in input.lines() {
        let (first, last) = find_parts(line)?;

        // Part 1
        if (first.contains(&last.start) && first.contains(&last.end))
            || (last.contains(&first.start) && last.contains(&first.end))
        {
            p1_score += 1
        }

        // Part 2
        if first.contains(&last.start)
            || first.contains(&last.end)
            || last.contains(&first.start)
            || last.contains(&first.end)
        {
            p2_score += 1
        }
    }
    Ok((p1_score, p2_score))
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let input = include_str!("../input");
    let (p1, p2) = score(input)?;
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn test_input() {
        assert_eq!(super::score(TEST_INPUT).unwrap(), (2, 4));
        assert_eq!(super::score(INPUT).unwrap(), (518, 909));
    }
}
