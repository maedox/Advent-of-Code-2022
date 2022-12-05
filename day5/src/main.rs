use crate::CraneModel::*;
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

fn get_stacks(input: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(...)\s(...)\s(...)\s(...)\s(...)\s(...)\s(...)\s(...)\s?(.*?)$")
                .unwrap();
    }
    let mut stacks = vec![
        // "".to_string(),
        /* 1 */ "".to_string(),
        /* 2 */ "".to_string(),
        /* 3 */ "".to_string(),
        /* 4 */ "".to_string(),
        /* 5 */ "".to_string(),
        /* 6 */ "".to_string(),
        /* 7 */ "".to_string(),
        /* 8 */ "".to_string(),
        /* 9 */ "".to_string(),
    ];

    for line in input.lines().take(8) {
        if let Some(groups) = RE.captures(line) {
            let strings: String = groups
                .iter()
                .skip(1)
                .map(|s| {
                    // Get rid of whitespace so we are only left with "crates"
                    if let Some(r) = s {
                        r.as_str().replace("   ", " ")
                    } else {
                        "".to_string()
                    }
                })
                .collect::<String>()
                // Remove [ ] around crates
                .replace(['[', ']'], "");

            (0..=8).for_each(|i| {
                if let Some(new_char) = strings.chars().nth(i) {
                    let curr = &stacks[i];
                    let new = format!("{}{}", curr, new_char);
                    stacks[i] = new.replace(' ', "");
                }
            });
        }
    }
    let stacks: Vec<String> = stacks
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    stacks
}

fn crane(input: &str, crane_model: CraneModel) -> String {
    let mut stacks = get_stacks(input);
    stacks.insert(0, "".to_string()); // easier indexing x_X

    // Hand-written stacks "cheat"
    // let mut stacks = vec![
    //     "".to_string(),
    //     /* 1 */ "ZPMHR".to_string(),
    //     /* 2 */ "PCJB".to_string(),
    //     /* 3 */ "SNHGLCD".to_string(),
    //     /* 4 */ "FTMDQSRL".to_string(),
    //     /* 5 */ "FSPQBTZM".to_string(),
    //     /* 6 */ "TFSZBG".to_string(),
    //     /* 7 */ "NRV".to_string(),
    //     /* 8 */ "PGLTDVCM".to_string(),
    //     /* 9 */ "WQNJFML".to_string(),
    // ];

    lazy_static! {
        // Example: move 7 from 3 to 9 => { num_crates=7, from=3, to=9 }
        static ref RE: Regex = Regex::new(
            r"move (?P<num_crates>\d{1,}) from (?P<from>\d) to (?P<to>\d)"
        )
        .unwrap();
    }
    for line in input.lines().skip(10) {
        if let Some(groups) = RE.captures(line) {
            let num_crates = groups["num_crates"].parse::<usize>().unwrap();
            let from = groups["from"].parse::<usize>().unwrap();
            let to = groups["to"].parse::<usize>().unwrap();

            let from_stack = stacks[from].clone();
            let to_stack = stacks[to].clone();

            // get the leftover and taken crates by slicing strings
            let slice_index = from_stack.len() - num_crates;
            let leftover = &from_stack[..slice_index];
            let taken_crates = &from_stack[slice_index..];

            let dest_stack = match crane_model {
                // Part 1: simulate grabbing one crate at a time
                CrateMover9000 => format!(
                    "{}{}",
                    to_stack,
                    taken_crates.chars().rev().collect::<String>()
                ),
                // Part 2: grab all crates simultaneously
                CrateMover9001 => format!("{}{}", to_stack, taken_crates),
            };

            stacks[from] = leftover.to_string();
            stacks[to] = dest_stack;
        }
    }

    // return the last/top crate in each stack, concatenated to a String.
    stacks
        .iter()
        .skip(1)
        .map(|s| s.chars().last().unwrap())
        .collect()
}

fn main() {
    println!("Part 1: {}", crane(INPUT, CrateMover9000));
    println!("Part 2: {}", crane(INPUT, CrateMover9001));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_input() {
        // part 1
        assert_eq!(crane(INPUT, CrateMover9000), "VQZNJMWTR");
        // part 2
        assert_eq!(crane(INPUT, CrateMover9001), "NLCDCLVMQ");
    }
}
