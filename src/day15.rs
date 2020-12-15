use std::collections::HashMap;

pub fn count(input: &str, n: usize) -> usize {
    let seed: Vec<usize> = input.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut map: HashMap::<usize, usize> = HashMap::new();
    let mut last_num: usize = 0;
    let mut next_num: usize = 0;
    for i in 0..n {
        last_num = match seed.get(i) {
            Some(v) => *v,
            None => next_num,
        };
        next_num = match map.get(&last_num) {
            Some(v) => i - v,
            None => 0,
        };
        map.insert(last_num, i);
    }
    last_num
}
// 150us
#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
    count(input, 2020)
}

// 3s :grimacing:
#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
    count(input, 30000000)
}
