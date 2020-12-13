use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// https://hellocode.dev/rust-lifetimes
// https://hellocode.dev/rust-ownership

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> i32 {
    lazy_static! {
        // Process each field:value token sucessively.
        // Two newlines in a row signals the end of a passport.
        static ref RULE_RE: Regex = Regex::new(r"(\w+ \w+) bags contain (.+\.)").unwrap();
        static ref BAGS_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?[,.]").unwrap();
    }
    let mut hash_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for caps in RULE_RE.captures_iter(input) {
        // https://stackoverflow.com/questions/51834111/lifetime-issue-iterating-over-regex-captures
        let parent = caps.get(1).unwrap().as_str();
        let child_str = caps.get(2).unwrap().as_str();
        // NB: Empty bags will not enter this loop, as child_str will equal "no other bags."
        for ccaps in BAGS_RE.captures_iter(child_str) {
            let child = ccaps.get(2).unwrap().as_str();
            if !hash_map.contains_key(child) {
                hash_map.insert(child, Vec::new());
            }
            // https://users.rust-lang.org/t/hashmap-with-vector-values/17906
            let parents = hash_map.get_mut(child).unwrap();
            parents.push(parent);
        }
    }
    let mut count = 0;
    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit: HashSet<&str> = HashSet::new();
    to_visit.insert("shiny gold");
    while !to_visit.is_empty() {
        // Why did I need a * here
        let child = *to_visit.iter().next().unwrap();
        to_visit.remove(child);
        if !hash_map.contains_key(child) {
            continue;
        }
        for parent in hash_map[child].iter() {
            if !visited.contains(parent) {
                count += 1;
                visited.insert(parent);
                to_visit.insert(parent);
            }
        }
    }
    count
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> i64 {
    lazy_static! {
        // Process each field:value token sucessively.
        // Two newlines in a row signals the end of a passport.
        static ref RULE_RE: Regex = Regex::new(r"(\w+ \w+) bags contain (.+\.)").unwrap();
        static ref BAGS_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?[,.]").unwrap();
    }
    let mut bag_to_content_count: HashMap<&str, i64> = HashMap::new();
    let mut parent_to_unprocessed: HashMap<&str, i64> = HashMap::new();
    let mut child_to_parents: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parent_to_contents: HashMap<&str, Vec<(i64, &str)>> = HashMap::new();
    // parent to # of unprocessed children
    // child to parents.
    // bag to # of bags it takes up.
    // parent to [{x, a}, {y, b}, {Z, c}]
    //
    // parent has X a, Y b, Z c
    // For each child that has no unprocessed children, set to 1.
    // For each parent of that child, decrement a counter. If that counter hits 0, add it to the children list.
    // Repeat until all nodes are processed.
    for caps in RULE_RE.captures_iter(input) {
        // https://stackoverflow.com/questions/51834111/lifetime-issue-iterating-over-regex-captures
        let parent = caps.get(1).unwrap().as_str();
        let child_str = caps.get(2).unwrap().as_str();

        parent_to_unprocessed.insert(parent, 0);
        parent_to_contents.insert(parent, Vec::new());

        // NB: Empty bags will not enter this loop, as child_str will equal "no other bags."
        for ccaps in BAGS_RE.captures_iter(child_str) {
            let child = ccaps.get(2).unwrap().as_str();
            let count = ccaps.get(1).unwrap().as_str().parse::<i64>().unwrap();

            if !child_to_parents.contains_key(child) {
                child_to_parents.insert(child, Vec::new());
            }
            // https://users.rust-lang.org/t/hashmap-with-vector-values/17906
            // TODO: would be nice to move these lookups to be part of 90 / 91
            child_to_parents.get_mut(child).unwrap().push(parent);
            *(parent_to_unprocessed.get_mut(parent).unwrap()) += 1;
            parent_to_contents
                .get_mut(parent)
                .unwrap()
                .push((count, child));
        }
    }
    // TODO: A HashSet isn't necessary since we never expect to insert duplicates.
    let mut cur_bags: HashSet<&str> = parent_to_unprocessed
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(parent, _)| *parent)
        .collect();
    while !cur_bags.is_empty() {
        let cur_bag = *cur_bags.iter().next().unwrap();
        cur_bags.remove(cur_bag);
        let cur_bag_count: i64 = parent_to_contents
            .get(cur_bag)
            .unwrap()
            .iter()
            .map(|(count, child)| {
                let content_bags = bag_to_content_count.get(child).unwrap();
                return count * (1 + content_bags);
            })
            .sum();
        bag_to_content_count.insert(cur_bag, cur_bag_count);
        if !child_to_parents.contains_key(cur_bag) {
            continue;
        }
        for parent in child_to_parents.get(cur_bag).unwrap() {
            let upc = parent_to_unprocessed.get_mut(parent).unwrap();
            *upc -= 1;
            if *upc == 0 {
                cur_bags.insert(parent);
            }
        }
    }
    *bag_to_content_count.get("shiny gold").unwrap()
}
