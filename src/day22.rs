use std::collections::VecDeque;
use std::collections::HashSet;

pub fn parse_deck(input: &str) -> VecDeque<i32> {
    let mut lines = input.lines();
    lines.next();
    lines.map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn score(deck: &VecDeque<i32>) -> i32 {
    deck.iter().enumerate().map(|(i, v)| ((deck.len() - i) as i32) * v).sum()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut deck_str_iter = input.split("\n\n");
    let mut deck1 = parse_deck(deck_str_iter.next().unwrap());
    let mut deck2 = parse_deck(deck_str_iter.next().unwrap());
    while deck1.len() > 0 && deck2.len() > 0 {
        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();
        if p1 > p2 {
            deck1.push_back(p1);
            deck1.push_back(p2);
        } else {
            deck2.push_back(p2);
            deck2.push_back(p1);
        }
    }
    if deck1.len() > 0 {
        score(&deck1)
    } else {
        score(&deck2)
    }
}


pub fn hash(deck1: &VecDeque<i32>, deck2: &VecDeque<i32>) -> String {
    let mut s = String::new();
    s += &deck1.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    s += ";";
    s += &deck2.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    s
}
pub fn play_subgame(mut deck1: VecDeque<i32>, mut deck2: VecDeque<i32>, depth: i32) -> [VecDeque<i32>; 2] {
    println!("recursing!");
    // println!("\n\nPlaying game with");
    // println!("P1: {:?}", deck1);
    // println!("P2: {:?}", deck2);

    let mut seen_games: HashSet<String> = HashSet::new();

    while deck1.len() > 0 && deck2.len() > 0 {
        println!("looping again: {} vs {}", deck1.len(), deck2.len());
        let h = hash(&deck1, &deck2);
        println!("h: {}", h);
        if seen_games.contains(&h) {
            println!("seen before!");
            return [deck1, VecDeque::new()]
        }
        println!("new round!");

        seen_games.insert(h);

        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();

        // If there are sufficient cards for both players,
        // play a subgame.
        let p1_wins = if p1 <= deck1.len() as i32 && p2 <= deck2.len() as i32 {
            println!("About to recurse");
            // subgame routine
            deck1.make_contiguous();
            deck2.make_contiguous();
            // println!("deck1: {} {} {:?}", p1, deck1.len(), deck1);
            // println!("deck2: {} {} {:?}", p2, deck2.len(), deck2);
            let resps = play_subgame(
                deck1.as_slices().0[0..p1 as usize].iter().map(|v| *v).collect(),
                deck2.as_slices().0[0..p2 as usize].iter().map(|v| *v).collect(),
                depth + 1
            );
            resps[0].len() > 0
        } else {
            println!("not enough to recurse, just declaring");
            p1 > p2
        };

        if p1_wins { 
            println!("p1 wins");
            deck1.push_back(p1);
            deck1.push_back(p2);
        } else {
            println!("p2 wins at depth={}", depth);
            deck2.push_back(p2);
            deck2.push_back(p1);
            println!("pushed {} - {} vs {}", depth, deck1.len(), deck2.len());
        }
    }
    println!("Exited the loop and returning the decks up a level");
    [deck1, deck2]
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut deck_str_iter = input.split("\n\n");
    let decks = play_subgame(
        parse_deck(deck_str_iter.next().unwrap()),
        parse_deck(deck_str_iter.next().unwrap()),
        0
    );
    println!("Game over!");
    if decks[0].len() > 0 {
        score(&decks[0])
    } else {
        score(&decks[1])
    }
}
