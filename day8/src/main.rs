const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", count_visible_trees(INPUT));
    println!("Part 2: {}", calculate_scenic_score(INPUT));
}

fn count_visible_trees(input: &str) -> usize {
    // make vec of vecs
    // count all sides, len of vecs, len of vecs[0]
    // for each vec 1..vec.len() -1 (exclude sides),
    // - check if any tree in same row/col is taller
    // - else count += 1
    let mut forest: Vec<Vec<u32>> = Vec::with_capacity(99);
    input.lines().for_each(|line| {
        forest.push(
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        );
    });

    // Actual size of forest X/Y
    let grid_size = forest.len();
    // Max index when iterating
    let iter_max_idx = grid_size - 1;

    // four sides of same length - four corners we shouldn't count twice
    let outer_trees: usize = grid_size * 4 - 4;
    let mut count: usize = dbg!(outer_trees);

    // for each row in forest
    // for each tree in row
    // - check if taller in same row
    // for each row in forest
    // - check if taller in each row at same index as current tree

    forest[..iter_max_idx]
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(row_idx, row)| {
            row[..iter_max_idx]
                .iter()
                .enumerate()
                .skip(1)
                .for_each(|(col_idx, tree)| {
                    // check to the left
                    if (row[..col_idx].iter()
                        .filter(|i| i >= &tree)
                        .count()
                        == 0)
                    ||

                    // check to the right
                    (row[col_idx + 1..=iter_max_idx].iter()
                        .filter(|i| i >= &tree)
                        .count()
                        == 0)
                    ||

                    // check above
                    (forest[..row_idx].iter()
                        .filter(|forest_row| forest_row[col_idx] >= *tree)
                        .count()
                        == 0)
                    ||

                    // check below
                    (forest[row_idx + 1..=iter_max_idx].iter()
                        .filter(|r| r[col_idx] >= *tree)
                        .count()
                        == 0)
                    {
                        count += 1
                    }
                })
        });

    count
    // count
}

fn calculate_scenic_score(input: &str) -> usize {
    let mut forest: Vec<Vec<u32>> = Vec::with_capacity(99);
    input.lines().for_each(|line| {
        forest.push(
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        );
    });

    // Actual size of forest X/Y
    let grid_size = forest.len();
    // Max index when iterating
    let iter_max_idx = grid_size - 1;

    // four sides of same length - four corners we shouldn't count twice
    let mut max_score: usize = 0;

    forest[..iter_max_idx]
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(row_idx, row)| {
            row[..iter_max_idx]
                .iter()
                .enumerate()
                .skip(1)
                .for_each(|(col_idx, tree)| {
                    // check to the left
                    let mut left = 0;
                    let rows = row[..col_idx].iter().rev();
                    for r in rows {
                        if r < tree {
                            left += 1
                        } else {
                            left += 1;
                            break;
                        }
                    }

                    // check to the right
                    let mut right = 0;
                    let rows = row[col_idx + 1..=iter_max_idx].iter();
                    for i in rows {
                        if i < tree {
                            right += 1
                        } else {
                            right += 1;
                            break;
                        }
                    }

                    // check above
                    let mut above = 0;
                    let rows = forest[..row_idx].iter().rev();
                    for forest_row in rows {
                        if forest_row[col_idx] < *tree {
                            above += 1;
                        } else {
                            above += 1;
                            break;
                        }
                    }

                    // check below
                    let mut below = 0;
                    let rows = forest[row_idx + 1..=iter_max_idx].iter();
                    for r in rows {
                        if r[col_idx] < *tree {
                            below += 1;
                        } else {
                            below += 1;
                            break;
                        }
                    }

                    let scenic_score = left * right * above * below;

                    if scenic_score > max_score {
                        max_score = scenic_score
                    }
                })
        });

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        assert_eq!(count_visible_trees(TEST_INPUT), 21);
        assert_eq!(count_visible_trees(INPUT), 1717);
    }

    #[test]
    fn part_2() {
        assert_eq!(calculate_scenic_score(TEST_INPUT), 8);
        assert_eq!(calculate_scenic_score(INPUT), 321975);
    }
}
