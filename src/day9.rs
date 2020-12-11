use std::collections::HashSet;
use std::collections::LinkedList;

const RUN: usize = 25;

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let mut list: LinkedList<i32> = LinkedList::new();
    input
        .lines()
        .map(|l| { l.parse::<i32>().unwrap() })
        .find(|x| {
            if list.len() >= RUN && list.iter().all(|a| !hash_set.contains(&(x - a))) {
                return true;
            }
            hash_set.insert(*x);
            list.push_back(*x);
            if list.len() > RUN {
                let y = list.pop_front().unwrap();
                hash_set.remove(&y);
            }
            return false;
        }).unwrap()
}


#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> i64 { 
    let numbers: Vec<i64> = input
        .lines()
        .map(|l| { l.parse::<i64>().unwrap() })
        .collect();

    let mut hash_set: HashSet<i64> = HashSet::new();
    let mut list: LinkedList<i64> = LinkedList::new();    
    let target = numbers.iter().find(|x| {
        if list.len() >= RUN && list.iter().all(|a| !hash_set.contains(&(*x - a))) {
            return true;
        }
        hash_set.insert(**x);
        list.push_back(**x);
        if list.len() > RUN {
            let y = list.pop_front().unwrap();
            hash_set.remove(&y);
        }
        return false;
    }).unwrap();
    println!("Target: {}", target);

    let mut first_iter = numbers.iter().enumerate();
    while let Some((i, v)) = first_iter.next() {
        let mut sum = *v;
        for (j, w) in first_iter.clone() {
            sum += w;
            if sum > *target {
                break;
            } 
            if sum == *target {
                let s: Vec<i64> = (i..j + 1).map(|k| *numbers.get(k as usize).unwrap()).collect();
                return *s.iter().min().unwrap() + *s.iter().max().unwrap()
            }
        }
    }
    panic!();
}

