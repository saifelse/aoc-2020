use std::collections::HashSet;

const ACTIVE: char = '#';


#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut hash_set: HashSet<(i32, i32, i32)> = HashSet::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == ACTIVE {
                hash_set.insert((i as i32, j as i32, 0));
            }
        }
    }
    let mut adj: Vec<(i32, i32, i32)> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if i != 1 || j != 1 || k != 1 {
                    adj.push((i - 1, j - 1, k - 1));
                }
            }
        }
    }

    for _ in 0..6 {
        let mut to_add: Vec<(i32, i32, i32)> = Vec::new();
        let mut to_rem: Vec<(i32, i32, i32)> = Vec::new();

        for (i, j, k) in hash_set.iter() {
            let mut count = 0;
            for (di, dj, dk) in adj.iter() {
                if hash_set.contains(&(i + di, j + dj, k + dk)) {
                    count += 1;
                    if count >= 4 {
                        to_rem.push((*i, *j, *k));
                        break;
                    }
                }
            }
            if count < 2 {
                to_rem.push((*i, *j, *k));
            }
        }

        let to_consider: HashSet<(i32, i32, i32)> = hash_set
            .iter()
            .flat_map(|(i, j, k)| adj.iter().map(move |(di, dj, dk)| (*i + *di, *j + *dj, *k + *dk)))
            .filter(|(i, j, k)| !hash_set.contains(&(*i, *j, *k)))
            .collect();
        for (i, j, k) in to_consider.iter() {
            let mut count = 0;
            for (di, dj, dk) in adj.iter() {
                if hash_set.contains(&(i + di, j + dj, k + dk)) {
                    count += 1;
                    if count >= 4 {
                        break;
                    }
                }
            }
            if count == 3 {
                to_add.push((*i, *j, *k));
            }
        }
        for x in to_add {
            hash_set.insert(x);
        }
        for x in to_rem {
            hash_set.remove(&x);
        }
    }
    hash_set.len()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut hash_set: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == ACTIVE {
                hash_set.insert((i as i32, j as i32, 0, 0));
            }
        }
    }
    let mut adj: Vec<(i32, i32, i32, i32)> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                for l in 0..3 {
                    if i != 1 || j != 1 || k != 1 || l != 1{
                        adj.push((i - 1, j - 1, k - 1, l - 1));
                    }
                }
            }
        }
    }

    for _ in 0..6 {
        let mut to_add: Vec<(i32, i32, i32, i32)> = Vec::new();
        let mut to_rem: Vec<(i32, i32, i32, i32)> = Vec::new();

        for (i, j, k, l) in hash_set.iter() {
            let mut count = 0;
            for (di, dj, dk, dl) in adj.iter() {
                if hash_set.contains(&(i + di, j + dj, k + dk, l + dl)) {
                    count += 1;
                    if count >= 4 {
                        to_rem.push((*i, *j, *k, *l));
                        break;
                    }
                }
            }
            if count < 2 {
                to_rem.push((*i, *j, *k, *l));
            }
        }

        let to_consider: HashSet<(i32, i32, i32, i32)> = hash_set
            .iter()
            .flat_map(|(i, j, k, l)| adj.iter().map(move |(di, dj, dk, dl)| (*i + *di, *j + *dj, *k + *dk, *l + *dl)))
            .filter(|(i, j, k, l)| !hash_set.contains(&(*i, *j, *k, *l)))
            .collect();
        for (i, j, k, l) in to_consider.iter() {
            let mut count = 0;
            for (di, dj, dk, dl) in adj.iter() {
                if hash_set.contains(&(i + di, j + dj, k + dk, l + dl)) {
                    count += 1;
                    if count >= 4 {
                        break;
                    }
                }
            }
            if count == 3 {
                to_add.push((*i, *j, *k, *l));
            }
        }
        for x in to_add {
            hash_set.insert(x);
        }
        for x in to_rem {
            hash_set.remove(&x);
        }
    }
    hash_set.len()
}
