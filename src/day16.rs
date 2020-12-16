use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref INPUT_RE: Regex =
        Regex::new(r"(?s)(.+)\n\nyour ticket:\n(.+)\n\nnearby tickets:\n(.+)").unwrap();
    static ref VALUE_RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Solver;
impl Solver {
    pub fn solve_constraints(mut constraints: &mut HashMap<usize, HashSet<usize>>) -> HashMap<usize, usize> {
        let mut sol: HashMap<usize, usize> = HashMap::new();
        while let Some((field_idx, col_idx)) = Solver::find_constrained_value(&constraints) {
            Solver::assign(field_idx, col_idx, &mut constraints, &mut sol);
        }
        assert!(constraints.len() == 0, "Some constraints were not solved!");
        return sol;
    }

    fn find_constrained_value(m: &HashMap<usize, HashSet<usize>>) -> Option<(usize, usize)> {
        match m.iter().find(|(_, y)| y.len() == 1) {
            Some((x, y)) => {
                let v = y.iter().next().unwrap();
                Some((*x, *v))
            }
            None => None,
        }
    }

    fn is_valid_assignment(range_idx: usize, col_idx: usize, constraints: &HashMap<usize, HashSet<usize>>) -> bool {
        constraints.get(&range_idx).unwrap().contains(&col_idx)
    }

    fn assign(range_idx: usize, col_idx: usize, constraints: &mut HashMap<usize, HashSet<usize>>, sol: &mut HashMap<usize, usize>) {
        sol.insert(range_idx, col_idx);
        constraints.remove(&range_idx);
        for (_, possive_values) in constraints.iter_mut() {
            possive_values.remove(&col_idx);
        }
    }

    pub fn brute_solve(constraints: &HashMap<usize, HashSet<usize>>) -> Option<HashMap<usize, usize>> {
        Solver::brute_solve_helper(&constraints, &HashMap::new())
    }
    
    pub fn brute_solve_helper(
        constraints: &HashMap<usize, HashSet<usize>>,
        sol: &HashMap<usize, usize>,
    ) -> Option<HashMap<usize, usize>> {
        // Solve for the most constrained key first.
        // TODO: Would be nice to maintain this data structure so we don't have to do it on every loop.
        let mut items: Vec<(&usize, &HashSet<usize>)> = constraints.iter().collect();
        items.sort_by_key(|(_, v1)| v1.len());
        // TODO: We could also look at the reverse mapping of constraints to see if any value is uniquely
        // constrained.
    
        // As efficient as find_constrained_value but more robust.
        match items.iter().next() {
            Some((r, possible_values)) =>  {
                for a in *possible_values {
                    if Solver::is_valid_assignment(**r, *a, constraints) {
                        // TODO: Abstract these away into a single struct
                        let mut new_constraints = constraints.clone();
                        let mut new_sol = sol.clone();
                        Solver::assign(**r, *a, &mut new_constraints, &mut new_sol);
                        let soln = Solver::brute_solve_helper(&new_constraints, &new_sol);
                        if let Some(v) = soln {
                            return Some(v.clone());
                        }
                    }
                }
                return None
            },
            None => Some(sol.clone())
        }
    }
}

type TicketRange = (String, (i32, i32), (i32, i32));
struct TicketData {
    ranges: Vec<TicketRange>,
    my_ticket: Vec<i32>,
    other_tickets: Vec<Vec<i32>>,
}

impl TicketData {
    pub fn parse(input: &str) -> TicketData {
        let caps = INPUT_RE.captures(input).unwrap();
        let ranges: Vec<TicketRange> = TicketData::parse_ranges(&caps[1]);
        let my_ticket: Vec<i32> = TicketData::parse_valid_tickets(&caps[2], &ranges)
            .first()
            .unwrap()
            .to_vec();
        let other_tickets: Vec<Vec<i32>> = TicketData::parse_valid_tickets(&caps[3], &ranges);
        TicketData {
            ranges,
            my_ticket,
            other_tickets,
        }
    }

    pub fn parse_valid_tickets(
        input: &str,
        ranges: &Vec<TicketRange>,
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
                    .all(|v| ranges.iter().any(|range| TicketData::is_a_valid_ticket_value(*v, range)))
            })
            .collect()
    }

    pub fn parse_ranges(input: &str) -> Vec<TicketRange> {
        lazy_static! {
            static ref RANGE_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }
        RANGE_RE
            .captures_iter(input)
            .map(|rcaps| {
                (
                    rcaps[1].to_string(),
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

    pub fn is_a_valid_ticket_value(v: i32, (_, (m, n), (o, p)): &TicketRange) -> bool {
        return *m <= v && v <= *n || *o <= v && v <= *p;
    }
    
    pub fn is_column_valid_for_field_range(
        tickets: &Vec<Vec<i32>>,
        range: &TicketRange,
        col_idx: usize,
    ) -> bool {
        tickets.iter().all(|ticket| TicketData::is_a_valid_ticket_value(ticket[col_idx], range))
    }

    pub fn to_constraints_map(&self) -> HashMap<usize, HashSet<usize>> {
        self.ranges
            .iter()
            .enumerate()
            .map(|(j, r)| {
                (
                    j,
                    (0..self.ranges.len())
                        .filter(|i| TicketData::is_column_valid_for_field_range(&self.other_tickets, r, *i))
                        .collect(),
                )
            })
            .collect()
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let caps = INPUT_RE.captures(input).unwrap();
    let ranges = TicketData::parse_ranges(&caps[1]);
    VALUE_RE
        .find_iter(&caps[3])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .filter(|v| {
            ranges
                .iter()
                .all(|r| !TicketData::is_a_valid_ticket_value(*v, r))
        })
        .sum()
}

pub fn calculate_part2(ranges: &Vec<TicketRange>, my_ticket: &Vec<i32>, sol: &HashMap<usize, usize>) -> i64 {
    ranges
        .iter()
        .enumerate()
        .filter(|(_, r)| r.0.starts_with("departure"))
        .map(|(i, _)| *my_ticket.get(*sol.get(&i).unwrap()).unwrap() as i64)
        .product()
}

#[aoc(day16, part2, no_brute)]
pub fn solve_part2_no_brute(input: &str) -> i64 {
    let ticket_data = TicketData::parse(input);
    let mut constraints = ticket_data.to_constraints_map();
    let sol = Solver::solve_constraints(&mut constraints);
    calculate_part2(&ticket_data.ranges, &ticket_data.my_ticket, &sol)
}

#[aoc(day16, part2, brute)]
pub fn solve_part2_brute(input: &str) -> i64 {
    let ticket_data = TicketData::parse(input);
    let constraints = ticket_data.to_constraints_map();
    let sol = Solver::brute_solve(&constraints).unwrap();
    calculate_part2(&ticket_data.ranges, &ticket_data.my_ticket, &sol)
}
