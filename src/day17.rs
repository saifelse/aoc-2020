use std::collections::HashSet;
use std::iter::once;
use std::iter::repeat;

const ACTIVE: char = '#';

pub fn adj_vec_helper(dim_n: i32) -> Vec<Vec<i32>> {
    match dim_n {
        0 => [Vec::<i32>::new()].iter().map(|v| v.clone()).collect(),
        _ => adj_vec_helper(dim_n - 1)
            .iter()
            .flat_map(|jk| {
                [-1, 1, 0].iter().map(move |i| {
                    jk.iter()
                        .map(|v| v.clone())
                        .chain(once(*i))
                        .collect::<Vec<i32>>()
                })
            })
            .collect(),
    }
}

pub fn adj_vec(dim_n: i32) -> Vec<Vec<i32>> {
    let mut adj = adj_vec_helper(dim_n);
    adj.remove(adj.len() - 1); // Remove (0, 0, 0)
    return adj;
}

pub fn parse(n: usize, input: &str) -> HashSet<Vec<i32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .filter(|(_, c)| *c == ACTIVE)
        .map(|((i, j), _)| {
            repeat(0)
                .take(n - 2)
                .chain(once(i as i32))
                .chain(once(j as i32))
                .collect()
        })
        .collect()
}

pub fn add(v: &Vec<i32>, dv: &Vec<i32>) -> Vec<i32> {
    v.iter().enumerate().map(|(i, x)| x + dv[i]).collect()
}

// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
// Otherwise, the cube becomes inactive.
pub fn compute_inactivate(active_set: &HashSet<Vec<i32>>, adj: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    active_set
        .iter()
        .filter(|v| {
            let c = adj
                .iter()
                .filter(|dv| active_set.contains(&add(&v, &dv)))
                .take(4)
                .count();
            c < 2 || c > 3
        })
        .map(|v| v.clone())
        .collect()
}

// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
pub fn compute_activate(active_set: &HashSet<Vec<i32>>, adj: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    active_set
        .iter()
        // Find all active-adjacent...
        .flat_map(|v| adj.iter().map(move |dv| add(v, dv)))
        // ...but inactive cells
        .filter(|v| !active_set.contains(v))
        // and deduplicate.
        .collect::<HashSet<Vec<i32>>>()
        .iter()
        // Then filter to those that meet the activation criteria.
        .filter(|v| {
            let c = adj
                .iter()
                .filter(|dv| active_set.contains(&add(&v, &dv)))
                .take(4)
                .count();
            c == 3
        })
        .map(|v| v.clone())
        .collect()
}

pub fn run(active_set: &mut HashSet<Vec<i32>>, adj: &Vec<Vec<i32>>) {
    let to_inactivate: Vec<Vec<i32>> = compute_inactivate(active_set, adj);
    let to_activate: Vec<Vec<i32>> = compute_activate(active_set, adj);
    for x in to_activate {
        active_set.insert(x);
    }
    for x in to_inactivate {
        active_set.remove(&x);
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut active_set = parse(3, input);
    let adj = adj_vec(3);
    for _ in 0..6 {
        run(&mut active_set, &adj);
    }
    active_set.len()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut active_set = parse(4, input);
    let adj = adj_vec(4);

    for _ in 0..6 {
        run(&mut active_set, &adj);
    }
    active_set.len()
}
