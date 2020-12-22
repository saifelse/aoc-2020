use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub enum Player {
    ONE,
    TWO,
}

pub struct Outcome {
    winner: Player,
    score: i32,
}

pub fn parse_deck(input: &str) -> VecDeque<i32> {
    let mut lines = input.lines();
    lines.next();
    lines.map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn score(deck: &VecDeque<i32>, depth: i32) -> i32 {
    if depth > 0 {
        0
    } else {
        deck.iter().enumerate().map(|(i, v)| ((deck.len() - i) as i32) * v).sum()
    }
}

pub fn hash(deck1: &VecDeque<i32>, deck2: &VecDeque<i32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    deck1.hash(&mut hasher);
    deck2.hash(&mut hasher);
    hasher.finish()
}


#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut deck_str_iter = input.split("\n\n");
    let mut deck1 = parse_deck(deck_str_iter.next().unwrap());
    let mut deck2 = parse_deck(deck_str_iter.next().unwrap());
    loop {
        if deck2.len() == 0 {
            return score(&deck1, 0);
        }
        if deck1.len() == 0 {
            return score(&deck2, 0);
        }
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

}

pub fn play_recursive_game(mut deck1: VecDeque<i32>, mut deck2: VecDeque<i32>, depth: i32) -> Outcome {
    let mut seen_games: HashSet<u64> = HashSet::new();
    // The game ends when one player has no cards
    loop {
        // 1a. Terminal conditions: game ends only if a player runs out of cards or the game is a repeat
        if deck2.len() == 0 {
            return Outcome {
                winner: Player::ONE,
                score: score(&deck1, depth),
            }
        } 
        if deck1.len() == 0 {
            return Outcome {
                winner: Player::TWO,
                score: score(&deck2, depth),
            }
        }
        // 1b. Even before drawing a card, let's check to see if we're caught in recursion.
        // Technically could have correctness issues for hash collisions, but it's not worth
        // the memory overhead.
        let h = hash(&deck1, &deck2);
        if seen_games.contains(&h) {
            return Outcome {
                winner: Player::ONE,
                score: score(&deck1, depth),
            }
        }
        seen_games.insert(h);
        // 2. Each player draws a card.
        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();
        // 3a. If there are sufficient cards for both players, play a subgame.
        let winner = if p1 <= deck1.len() as i32 && p2 <= deck2.len() as i32 {
            play_recursive_game(
                deck1.iter().take(p1 as usize).map(|v| *v).collect(),
                deck2.iter().take(p2 as usize).map(|v| *v).collect(),
                depth + 1
            ).winner
        // 3b. Otherwise, whoever drew the higher card wins
        } else {
            if p1 > p2 {
                Player::ONE
            } else {
                Player::TWO
            }
        };
        // 4. Winner collects the cards into their deck
        match winner {
            Player::ONE => {
                deck1.push_back(p1);
                deck1.push_back(p2);
            },
            Player::TWO => {
                deck2.push_back(p2);
                deck2.push_back(p1);
            }
        }
    }
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut deck_str_iter = input.split("\n\n");
    let outcome = play_recursive_game(
        parse_deck(deck_str_iter.next().unwrap()),
        parse_deck(deck_str_iter.next().unwrap()),
        0
    );
    outcome.score
}
