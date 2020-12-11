// use regex::Regex;
// use lazy_static::lazy_static;
// use std::collections::HashMap;


#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> i32 {
    // lazy_static! {
    //     // Process each field:value token sucessively.
    //     // Two newlines in a row signals the end of a passport.
    //     static ref RULE_RE: Regex = Regex::new(r"(\w+ \w+) bags contain (.+)\.").unwrap();
    //     static ref BAGS_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?[,.]").unwrap();
    // }
    // let mut hash_map: HashMap<&str, &mut Vec<&str>> = HashMap::new();
    
    // RULE_RE.captures_iter(input).map(|caps| {
    //     let parent = &caps[1];
    //     let child_str = &caps[2];
    //     for ccaps in BAGS_RE.captures_iter(child_str) {
    //         let child = &ccaps[2];
    //         // if (!hash_map.contains_key(child)) {
    //         //     hash_map.insert(child, Vec::new());
    //         // }
    //         let parents = &hash_map.get(child).unwrap();
    //         parents.push("hello!"); //.push(parent);
    //     }
    // });
    
    // let mut count = 0;
    // let mut visited = HashSet::new();
    // let mut to_visit = HashSet::new();
    // to_visit.insert("shiny gold");
    // while to_visit.len() > 0 {
    //     let child = to_visit.iter().next().unwrap();
    //     to_visit.remove(child);
    //     for parent in hash_map[child].iter() {
    //         if !visited.contains(parent) {
    //             count += 1;
    //             visited.insert(parent);
    //             to_visit.insert(parent);
    //         }
    //     }
    // }
    // count
    1
}

