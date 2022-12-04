// A, X == rock, B, Y == paper, C, Z = scissors

// A beats Z, // X beats C
// B beats X, // Y beats A
// C beats Y, // Z beats B

fn main() {
    let input = include_str!("../input");

    // PART 1
    let mut score = 0;
    for line in input.lines() {
        match line {
            "B X" => score += 1,
            "C Y" => score += 2,
            "A Z" => score += 3,
            "A X" => score += 4,
            "B Y" => score += 5,
            "C Z" => score += 6,
            "C X" => score += 7,
            "A Y" => score += 8,
            "B Z" => score += 9,
            _ => {
                panic!("Line: '{:?}'. This shouldn't happen ...", line)
            }
        }
    }
    println!("Part 1: Final score: {}", score);

    // PART 2
    let mut score = 0;
    for line in input.lines() {
        match line {
            "B X" => score += 1, // X
            "C X" => score += 2, // Y
            "A X" => score += 3, // Z
            "A Y" => score += 4, // X
            "B Y" => score += 5, // Y
            "C Y" => score += 6, // Z
            "C Z" => score += 7, // X
            "A Z" => score += 8, // Y
            "B Z" => score += 9, // Z
            _ => {
                panic!("Line: '{:?}'. This shouldn't happen ...", line)
            }
        };
    }
    println!("Part 2: Final score: {}", score);
}
