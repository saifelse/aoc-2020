use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> i32 {
    lazy_static! {
        static ref INPUT_RE: Regex =
            Regex::new(r"(?s)(.+)\n\nyour ticket:(.+)\n\nnearby tickets:(.+)").unwrap();
        static ref VALUE_RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let caps = INPUT_RE.captures(input).unwrap();
    let ranges = parse_ranges(&caps[1]);
    VALUE_RE
        .find_iter(&caps[3])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .filter(|v| {
            ranges
                .iter()
                .all(|r| !is_a_valid_ticket_value(*v, *r))
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> i64 {
    lazy_static! {
        static ref INPUT_RE: Regex =
            Regex::new(r"(?s)(.+)\n\nyour ticket:\n(.+)\n\nnearby tickets:\n(.+)").unwrap();
    }
    let caps = INPUT_RE.captures(input).unwrap();
    let ranges: Vec<(&str, (i32, i32), (i32, i32))> = parse_ranges(&caps[1]);
    let my_ticket: Vec<i32> = parse_valid_tickets(&caps[2], &ranges)
        .first()
        .unwrap()
        .to_vec();
    let other_tickets: Vec<Vec<i32>> = parse_valid_tickets(&caps[3], &ranges);
    let mut constraints = build_base_constraints(&ranges, &other_tickets);
    let sol = solve_constraints(&mut constraints);
    ranges
        .iter()
        .enumerate()
        .filter(|(_, r)| r.0.starts_with("departure"))
        .map(|(i, _)| *my_ticket.get(*sol.get(&i).unwrap()).unwrap() as i64)
        .product()
}

pub fn build_base_constraints(ranges: &Vec<(&str, (i32, i32), (i32, i32))>, other_tickets: &Vec<Vec<i32>>) -> HashMap<usize, HashSet<usize>> {
    ranges
        .iter()
        .enumerate()
        .map(|(j, r)| {
            (
                j,
                (0..ranges.len())
                    .filter(|i| is_column_valid_for_field_range(other_tickets, *r, *i))
                    .collect(),
            )
        })
        .collect()
}

pub fn solve_constraints(
    constraints: &mut HashMap<usize, HashSet<usize>>,
) -> HashMap<usize, usize> {
    let mut sol: HashMap<usize, usize> = HashMap::new();
    // Consider a few techniques for assigning each key in constraints to a unique value in the value set:
    // 1. If the possible values for a given key has 1 element, we can assign that value to the key.
    // 2. If a value is uniquely included for a possibility in one key, we can assign that value.
    // 3. Otherwise, we should brute force *try* assigning a value to the most constrained key, and then
    //    recursing.
    // As it turns out... (1) is sufficient for the provided input, so (2) and (3) are not implemented.
    while let Some((x, y)) = find_constrained_value(&constraints) {
        sol.insert(x, y);
        constraints.remove(&x);
        for (_, possive_values) in constraints.iter_mut() {
            possive_values.remove(&y);
        }
    }
    match constraints.len() {
        0 => sol,
        _ => panic!(), // TODO: This case is possible, but we don't run into it.
    }
}

pub fn find_constrained_value(m: &HashMap<usize, HashSet<usize>>) -> Option<(usize, usize)> {
    match m.iter().find(|(_, y)| y.len() == 1) {
        Some((x, y)) => {
            let v = y.iter().next().unwrap();
            Some((*x, *v))
        }
        None => None,
    }
}


pub fn parse_ranges(input: &str) -> Vec<(&str, (i32, i32), (i32, i32))> {
    lazy_static! {
        static ref RANGE_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    RANGE_RE
        .captures_iter(input)
        .map(|rcaps| {
            (
                rcaps.get(1).unwrap().as_str(),
                (
                    rcaps[2].parse::<i32>().unwrap(),
                    rcaps[3].parse::<i32>().unwrap(),
                ),
                (
                    rcaps[4].parse::<i32>().unwrap(),
                    rcaps[5].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect()
}

pub fn parse_valid_tickets(
    input: &str,
    ranges: &Vec<(&str, (i32, i32), (i32, i32))>,
) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|r| {
            r.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|t| {
            t.iter()
                .all(|v| ranges.iter().any(|range| is_a_valid_ticket_value(*v, *range)))
        })
        .collect()
}


pub fn is_a_valid_ticket_value(v: i32, (_, (m, n), (o, p)): (&str, (i32, i32), (i32, i32))) -> bool {
    return m <= v && v <= n || o <= v && v <= p;
}

pub fn is_column_valid_for_field_range(
    tickets: &Vec<Vec<i32>>,
    range: (&str, (i32, i32), (i32, i32)),
    col_idx: usize,
) -> bool {
    tickets.iter().all(|ticket| is_a_valid_ticket_value(ticket[col_idx], range))
}
