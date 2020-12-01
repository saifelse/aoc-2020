use std::collections::HashSet;


#[aoc(day1, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let hash_set: HashSet<i32> = input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .collect();
    for v in &hash_set {
        let w: i32 = 2020 - v;
        if hash_set.contains(&w) {
            return v * w;
        }
    }
    unreachable!()
}


#[aoc(day1, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let hash_set: HashSet<i32> = input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .collect();
    for v in &hash_set {
        for w in &hash_set {
            let x: i32 = 2020 - v - w;
            if hash_set.contains(&x) {
                return v * w * x;
            }
        }
    }
    unreachable!()
}


// Sanity checking that the naive solution is in fact the slowest.
#[aoc(day1, part1, for_loop)]
pub fn solve_part1_for(input: &str) -> i32 {
    let int_vec: Vec<i32> = input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .collect();
    for v in &int_vec {
        for w in &int_vec {
            if v + w == 2020 {
                return v * w;
            }
        }
    }
    unreachable!()
}

// Sanity checking that the naive solution is in fact the slowest.
#[aoc(day1, part2, for_loop)]
pub fn solve_part2_for(input: &str) -> i32 {
    let int_vec: Vec<i32> = input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .collect();
    for v in &int_vec {
        for w in &int_vec {
            for x in &int_vec {
                if v + w + x == 2020 {
                    return v * w * x;
                }
            }
        }
    }
    unreachable!()
}


// Instead of iterating over n^2 pairs, we can do n(n-1)/2 by starting after
// the current elment. This is about 16% faster than for_hash soution.
#[aoc(day1, part2, iterclone)]
pub fn solve_part2_iterclone(input: &str) -> i32 {
    let hash_set: HashSet<i32> = input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .collect();
    
    // I wonder if it would be faster if we had a list and used array indexes.
    let mut first_iter = hash_set.iter();
    while let Some(v) = first_iter.next() {
        for w in first_iter.clone() {
            let x: i32 = 2020 - v - w;
            if hash_set.contains(&x) {
                return v * w * x;
            }
        }
    }
    unreachable!()
}
