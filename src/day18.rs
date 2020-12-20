use  std::str::Chars;

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> u64 {
    input.lines().map(|l| {
        let e = evaluate(&mut l.chars());
        println!("{} = {}", l, e);
        return e;
    }).sum()
}

pub fn next_value(chars: &mut Chars<'_>) -> u64 {
    let c = chars.next().unwrap();
    match c {
        '(' => evaluate(chars),
        ' ' => next_value(chars),
        _ => c.to_digit(10).unwrap().into(),
    }
}

pub fn evaluate(chars: &mut Chars<'_>) -> u64 {
    let mut value: u64 = next_value(chars);
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                value += next_value(chars);
            },
            '*' => {
                value *= next_value(chars);
            }
            ' ' => {
                continue;
            }
            ')' => {
                return value;
            }
            _ => {
                panic!();
            }
        }
    }
    return value;
}

pub enum Operator {
    ADD,
    MUL,
}

pub fn op_evaluate(chars: &mut Chars<'_>) -> u64 {
    let mut op_stack: Vec<Operator> = Vec::new();
    let mut operand_stack: Vec<u64> = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                op_stack.push(Operator::ADD);
            },
            '*' => {
                op_stack.push(Operator::MUL);
            }
            ' ' => {
                continue;
            }
            '(' => {
                // Evaluate a nested expression and put it on operand stack.
                operand_stack.push(op_evaluate(chars));
                run_op_with_precedence(&mut op_stack, &mut operand_stack);
            }
            ')' => {
                // End of an expression... probably should make sure the stack is empty.
                run(&mut op_stack, &mut operand_stack);
                return *operand_stack.iter().next().unwrap();
            }
            _ => {
                let v = c.to_digit(10).unwrap().into();
                operand_stack.push(v);
                run_op_with_precedence(&mut op_stack, &mut operand_stack);
            }
        }
    }
    run(&mut op_stack, &mut operand_stack);
    *operand_stack.iter().next().unwrap()
}

pub fn run(mut op_stack: &mut Vec<Operator>, mut operand_stack: &mut Vec<u64>) {
    while run_op_with_precedence(&mut op_stack, &mut operand_stack) {
    }
    run_op(&mut op_stack, &mut operand_stack);
}

pub fn run_op_with_precedence(mut op_stack: &mut Vec<Operator>, mut operand_stack: &mut Vec<u64>) -> bool {
    let mut op_pop = op_stack.iter().rev();
    match (op_pop.next(), op_pop.next()) {
        (_, None) => {
            return false;
        },
        (Some(Operator::ADD), Some(Operator::MUL)) => {
            run_op(&mut op_stack, &mut operand_stack);
            return true;
        },
        _ => {
            let op = op_stack.pop().unwrap();
            let v = operand_stack.pop().unwrap();
            run_op(&mut op_stack, &mut operand_stack);
            op_stack.push(op);
            operand_stack.push(v);
            return true;
        }
    }
}

pub fn run_op(op_stack: &mut Vec<Operator>, operand_stack: &mut Vec<u64>) -> bool {
    match op_stack.pop() {
        Some(op) => {
            let r = operand_stack.pop().unwrap();
            let l = operand_stack.pop().unwrap();
            match op {
                Operator::MUL => operand_stack.push(l * r),
                Operator::ADD => operand_stack.push(l + r),
            }
            return true;
        },
        None => {
            return false;
        }
    }
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> u64 {
    input.lines().map(|l| op_evaluate(&mut l.chars())).sum()
}

