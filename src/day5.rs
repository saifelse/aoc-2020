use itertools::sorted;
use std::clone::Clone;
use std::collections::HashSet;
use std::iter::Iterator;

trait CloneIterator: Iterator + Clone {}

pub fn seat_to_id(seat: &str) -> i32 {
    // This is just a binary encoding described in a very roundabout way...
    // BFFFBBFRRR --> 0b1000110111 --> 567
    // FFFBBBFRRR --> 0b0001110111 --> 119
    // BBFFBBFRLL --> 0b1100110100 --> 820
    seat.chars().fold(0, |acc, b| {
        acc * 2
            + match b {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            }
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.lines().map(|l| seat_to_id(l)).max().unwrap()
}

// This doesn't work because types and memory management are hard.
// #[aoc(day5, part1, vscan)]
// pub fn solve_part1_vscan(input: &str) -> i32 {
//     let data = input.as_bytes();
//     let mut seat_ids: Vec<i32> = Vec::new();
//     const width: usize = 10;
//     let rows = (data.len() + 1) / (width + 1);
//     let it: Box<dyn CloneIterator<Item = i32>> = Box::new((0..(rows+1)));
//     for j in 0..width {
//         let mut maybe_next_iter = it.clone().filter(|i| matches!(data[i * (width + 1) + j], b'B' | b'R')).peekable();
//         if maybe_next_iter.peek().is_some() {
//             it = &maybe_next_iter;
//         }
//     }
//     it.next().unwrap()
// }

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut seat_ids: Vec<i32> = input.lines().map(|l| seat_to_id(l)).collect();
    seat_ids.sort();
    let mut iter = seat_ids.iter().peekable();
    while let Some(seat_id) = iter.next() {
        if let Some(next_seat_id) = iter.peek() {
            if seat_id + 2 == **next_seat_id {
                return seat_id + 1;
            }
        }
    }
    panic!();
}

// This is slower than the above.
#[aoc(day5, part2, hashset)]
pub fn solve_part2_hashset(input: &str) -> i32 {
    let taken: HashSet<i32> = input.lines().map(|l| seat_to_id(l)).collect();
    (0..1024)
        .find(|x| taken.contains(&(x - 1)) && !taken.contains(&x) && taken.contains(&(x + 1)))
        .unwrap()
}

// Basically as fast as solve_part2
#[aoc(day5, part2, window)]
pub fn solve_part2_window(input: &str) -> i32 {
    sorted(input.lines().map(|l| seat_to_id(l)))
        .collect::<Vec<i32>>()
        .windows(2)
        .find(|w| w[1] - w[0] == 2)
        .unwrap()[0]
        + 1
}
