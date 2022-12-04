fn main() {
    // input contains elves carrying calories.
    // Each elf is separated by an empty line.

    // Part 1.
    // Sum the calories for each elf and return the one carrying the most.
    // Part 2.
    // Sum the calories of the top 3 elves and return the result.
    let mut elves: Vec<i32> = vec![];
    let mut curr = 0;
    let input = include_str!("../input");
    for line in input.lines() {
        if line.is_empty() {
            elves.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap_or_default();
        }
    }
    elves.sort();
    let result: &i32 = &elves.iter().rev().take(3).sum();
    println!("{}", result);
}
