use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// TODO: Break this down into saner steps
pub fn get_data(input: &str) -> (HashMap<String, i32>, HashMap<String, HashSet<String>>) {
    lazy_static! {
        static ref LINE_RE: Regex =
            Regex::new(r"(.+) \(contains (.+)\)").unwrap();
        static ref VALUE_RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut non_allergen_ingredient_frequency: HashMap<String, i32> = HashMap::new();
    let mut allergen_to_possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let caps = LINE_RE.captures(line).unwrap();
        let ingredients: HashSet<String> = caps[1].split(' ').map(|s| s.to_string()).collect();
        let allergens: Vec<&str> = caps[2].split(", ").collect();
        for allergen in allergens {
            if !allergen_to_possible_ingredients.contains_key(allergen) {
                let poss: HashSet<String> = ingredients.iter().map(|s| s.to_string()).collect();
                allergen_to_possible_ingredients.insert(allergen.to_string(), poss);
            } else {
                allergen_to_possible_ingredients.get_mut(allergen).unwrap().retain(|k| ingredients.contains(k));
            }
        }
        for ingredient in ingredients {
            *non_allergen_ingredient_frequency.entry(ingredient.to_string()).or_insert(0) += 1;
        }
    }
    for ings in allergen_to_possible_ingredients.values() {
        for ing in ings {
            non_allergen_ingredient_frequency.remove(&ing.to_string());
        }
    }
    (non_allergen_ingredient_frequency, allergen_to_possible_ingredients)
}

// TOOD: Generalize the code from day16.rs

pub fn assign(range_idx: String, col_idx: String, constraints: &mut HashMap<String, HashSet<String>>, sol: &mut HashMap<String, String>) {
    sol.insert(range_idx.clone(), col_idx.clone());
    constraints.remove(&range_idx);
    for (_, possive_values) in constraints.iter_mut() {
        possive_values.remove(&col_idx);
    }
}

pub fn solve_constraints(
    constraints: &mut HashMap<String, HashSet<String>>,
) -> HashMap<String, String> {
    let mut sol: HashMap<String, String> = HashMap::new();
    while let Some((field_idx, col_idx)) = find_constrained_value(&constraints) {
        assign(field_idx, col_idx, constraints, &mut sol);
    }
    match constraints.len() {
        0 => sol,
        _ => panic!(), // TODO: This case is possible, but it turns out every every constraint is uniquely identified.
    }
}

pub fn find_constrained_value(m: &HashMap<String, HashSet<String>>) -> Option<(String, String)> {
    match m.iter().find(|(_, y)| y.len() == 1) {
        Some((x, y)) => {
            let v = y.iter().next().unwrap();
            Some((x.clone(), v.clone()))
        }
        None => None,
    }
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> i32 {
    get_data(input).0.values().sum()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> String {
    let mut allergen_to_ing = get_data(input).1;
    let sol = solve_constraints(&mut allergen_to_ing);
    let mut sorted_allergens: Vec<String> = sol.keys().map(|s| s.clone()).collect();
    sorted_allergens.sort();
    let sol: Vec<&str> = sorted_allergens.iter().map(|a| sol.get(a).unwrap().as_str()).collect();
    sol.join(",")
}