use itertools::sorted;
use std::iter::once;


#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    sorted(
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            // Implicit outlet has 0 jolt rating 
            .chain(once(0)),
    )
    .collect::<Vec<i32>>()
    // Compuate joltage differences
    .windows(2)
    .map(|w| w[1] - w[0])
    // Implicit device's built-in adapter is +3
    .chain(once(3))
    // Accumulate a tuple of [count of diff 1, count of diff 3]
    .fold([0; 2], |acc, d| match d {
        1 => [acc[0] + 1, acc[1]],
        2 => acc,
        3 => [acc[0], acc[1] + 1],
        _ => panic!(),
    })
    .iter()
    // Take the product of (count of diff 1) x (count of diff 2)
    .product()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i64 {
    sorted(
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            // Implicit outlet has 0 jolt rating 
            .chain(once(0)),
    )
    .collect::<Vec<i32>>()
    // Compute joltage differences
    .windows(2)
    .map(|w| w[1] - w[0])
    // Implicit device's built-in adapter is +3
    .chain(once(3))
    // acc is a tuple of running counts where acc[x] is the number of ways
    // one can get to the current delta `d` with an active group having sum `x`.
    .fold([1, 0, 0], |acc, d| match d {
        1 => [acc[2], acc.iter().sum(), acc[1]],
        2 => [acc[1], 0, acc.iter().sum()],
        3 => [acc.iter().sum(), 0, 0],
        _ => panic!(),
    })
    .iter()
    .sum()
}
