#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut voltages: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    voltages.push(0);
    voltages.sort();
    let mut iter = voltages.iter().peekable();
    let mut count1: i32 = 0;
    let mut count3: i32 = 0;
    while let Some(voltage) = iter.next() {
        if let Some(next_voltage) = iter.peek() {
            match *next_voltage - *voltage {
                1 => count1 += 1,
                3 => count3 += 1,
                _ => panic!()
            }
        }
    }
    count3 += 1;
    return count1 * count3;
}


#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut voltages: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    voltages.push(0);
    voltages.push(voltages.iter().max().unwrap() + 3);
    voltages.sort();
    let deltas = &voltages.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let mut agg0:i64 = 1;
    let mut agg1:i64 = 0;
    let mut agg2:i64 = 0;

    for delta in deltas.iter() {
        let new_agg0:i64;
        let new_agg1:i64;
        let new_agg2:i64;
        match delta {
            1 => {
                new_agg0 = /* add 1 */ agg2;
                new_agg1 = /* terminate and start new */ agg0 + agg1 + agg2;
                new_agg2 = /* add 1 */ agg1;
            },
            2 => {
                new_agg0 = /* add 2 */ agg1;
                new_agg1 = 0;
                new_agg2 = /* terminate and start new */ agg0 + agg1 + agg2
            },
            3 => {
                new_agg0 = /* terminate and start new */ agg0 + agg1 + agg2;
                new_agg1 = 0;
                new_agg2 = 0;
            },
            _ => panic!(),
        }
        agg0 = new_agg0;
        agg1 = new_agg1;
        agg2 = new_agg2;
    }
    return agg0 + agg1 + agg2;
}
