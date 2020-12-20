// https://bryce.fisher-fleig.org/strategies-for-returning-references-in-rust/
// http://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special/
// https://medium.com/@KevinHoffman/to-box-or-not-to-box-my-first-real-rust-refactor-db467119c4c7

use regex::Regex;
use std::collections::HashMap;
use string_builder;
use topological_sort::TopologicalSort;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Or,
    Then,
    RuleRef,
    LiteralStr,
    Empty,
}

// TODO: Can we use Cow to make this data structure cheaper? We are basically never mutating anything.
// Is there a better way to support different Kinds having different fields?
#[derive(Debug, Clone)]
pub struct Op {
    kind: Kind,
    left: Option<Box<Op>>,
    right: Option<Box<Op>>,
    rule_id: Option<i32>,
    str_val: Option<String>,
    quantifier: Option<String>,
}

pub struct Rule {
    id: i32,
    op: Op,
}

pub fn rep(count: i32) -> String {
    let mut s = String::new();
    s += "{";
    s += &count.to_string();
    s += "}";
    s
}

impl Op {
    pub fn empty() -> Op {
        Op {
            kind: Kind::Empty,
            left: None,
            right: None,
            str_val: None,
            rule_id: None,
            quantifier: None,
        }
    }

    pub fn rule(rule_id: i32, quantifier: Option<String>) -> Op {
        Op {
            kind: Kind::RuleRef,
            left: None,
            right: None,
            rule_id: Some(rule_id),
            str_val: None,
            quantifier,
        }
    }

    pub fn then(&self, op: &Op) -> Op {
        match (self.kind, op.kind) {
            (_, Kind::Empty) => self.clone(),
            (Kind::Empty, _) => op.clone(),
            _ => Op {
                kind: Kind::Then,
                left: Some(Box::new(self.clone())),
                right: Some(Box::new(op.clone())),
                rule_id: None,
                str_val: None,
                quantifier: None,
            },
        }
    }

    pub fn or(&self, op: &Op) -> Op {
        match (self.kind, op.kind) {
            (_, Kind::Empty) => self.clone(),
            (Kind::Empty, _) => op.clone(),
            _ => Op {
                kind: Kind::Or,
                left: Some(Box::new(self.clone())),
                right: Some(Box::new(op.clone())),
                rule_id: None,
                str_val: None,
                quantifier: None,
            },
        }
    }

    pub fn to_regex_str(&self, lookup_table: &HashMap<i32, String>) -> String {
        let mut builder = string_builder::Builder::new(0);
        self.build(&lookup_table, &mut builder);
        return builder.string().unwrap();
    }

    pub fn build(
        &self,
        lookup_table: &HashMap<i32, String>,
        builder: &mut string_builder::Builder,
    ) {
        builder.append("(?:");
        match self.kind {
            Kind::Or => {
                self.left.as_ref().unwrap().build(lookup_table, builder);
                builder.append("|");
                self.right.as_ref().unwrap().build(lookup_table, builder);
            }
            Kind::Then => {
                self.left.as_ref().unwrap().build(lookup_table, builder);
                self.right.as_ref().unwrap().build(lookup_table, builder);
            }
            Kind::RuleRef => match lookup_table.get(&self.rule_id.unwrap()) {
                Some(str) => builder.append(str.clone()),
                None => {
                    panic!("Referenced rule has not been compiled yet!");
                }
            },
            Kind::LiteralStr => {
                builder.append(self.str_val.as_ref().unwrap().as_str());
            }
            Kind::Empty => {
                panic!("Empty nodes cannot be compiled!");
            }
        }
        builder.append(")");
        if let Some(q) = &self.quantifier {
            builder.append(q.clone());
        }
    }

    pub fn parse(rule_def: &str) -> Op {
        return rule_def
            .split(" | ")
            .map(|rule| {
                rule.split(' ')
                    .map(|term| {
                        if term.starts_with('"') {
                            Op {
                                kind: Kind::LiteralStr,
                                left: None,
                                right: None,
                                rule_id: None,
                                str_val: Some(term.trim_matches('"').to_string()),
                                quantifier: None,
                            }
                        } else {
                            Op {
                                kind: Kind::RuleRef,
                                left: None,
                                right: None,
                                str_val: None,
                                rule_id: Some(term.parse::<i32>().unwrap()),
                                quantifier: None,
                            }
                        }
                    })
                    .fold(Op::empty(), move |acc, op| acc.then(&op))
            })
            .fold(Op::empty(), move |acc, op| acc.or(&op));
    }

    pub fn get_rule_deps(&self) -> impl Iterator<Item = i32> + '_ {
        // NB: This could be safely cached if an Operator is immutable
        // https://stackoverflow.com/questions/29760668/conditionally-iterate-over-one-of-several-possible-iterators
        let iter: Box<dyn Iterator<Item = i32>> = match self.kind {
            Kind::RuleRef => Box::new(std::iter::once(self.rule_id.unwrap())),
            Kind::Or | Kind::Then => Box::new(
                self.left
                    .as_ref()
                    .unwrap()
                    .get_rule_deps()
                    .chain(self.right.as_ref().unwrap().get_rule_deps()),
            ),
            _ => Box::new(std::iter::empty::<i32>()),
        };
        return iter;
    }
}

pub fn parse_rules(rules_str: &str) -> HashMap<i32, Rule> {
    rules_str
    .lines()
    .map(|l| {
        let mut y = l.split(": ");
        let rule_id = y.next().unwrap().parse::<i32>().unwrap();
        let rule_def = y.next().unwrap();
        let op = Op::parse(rule_def);
        let rule_op = Rule { id: rule_id, op };
        (rule_id, rule_op)
    })
    .collect()
}

pub fn build_lookup(rules: &HashMap<i32, Rule>) -> HashMap<i32, String> {
    // Build a topological map of the dependencies in rules
    let mut ts = TopologicalSort::<i32>::new();
    for rule in rules.values() {
        for dep_id in rule.op.get_rule_deps() {
            ts.add_dependency(dep_id, rule.id);
        }
    }
    // Translate all rules into regexes
    let mut lookup: HashMap<i32, String> = HashMap::new();
    while let Some(rule_id) = ts.pop() {
        let rule = rules.get(&rule_id).unwrap();
        let reg = rule.op.to_regex_str(&lookup);
        lookup.insert(rule_id, reg);
    }
    return lookup;
}

pub fn get_regex(lookup: &HashMap<i32, String>, rule_id: i32) -> Regex {
    let mut z = String::new();
    z += "^(";
    z += lookup.get(&rule_id).unwrap();
    z += ")$";
    return Regex::new(&z).unwrap();
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let examples_str = sections.next().unwrap();
    let rules: HashMap<i32, Rule> = parse_rules(rules_str);
    let lookup = build_lookup(&rules);
    let zero_re = get_regex(&lookup, 0);
    examples_str
        .lines()
        .filter(|l| zero_re.is_match(*l))
        .count() as i32
}


#[aoc(day19, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let examples_str = sections.next().unwrap();
    let mut rules: HashMap<i32, Rule> = parse_rules(rules_str);
    rules.insert(8, Rule {id: 8, op: Op::rule(42, Some("+".to_string()))});
    rules.insert(11, Rule {
        id: 11,
        op:
            // Arbitrarily chose 10 :grimacing: ... Regex won't compile if this value is too big.
            (1..10)
                .map(|n| Op::rule(42, Some(rep(n))).then(&Op::rule(31, Some(rep(n)))))
                .fold(Op::empty(), |acc, op| acc.or(&op)),
    });
    let lookup = build_lookup(&rules);
    let zero_re = get_regex(&lookup, 0);

    examples_str
        .lines()
        .filter(|l| zero_re.is_match(*l))
        .count() as i32
}