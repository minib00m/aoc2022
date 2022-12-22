use std::io::{self, BufRead};

fn pick_score(pick: char) -> u32
{
    match pick {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0
    }
}

fn battle_score(theirs: char, ours: char) -> u32
{
    let alt_ours: char = (ours as u8 - (b'X' - b'A')) as char;
    match (theirs, alt_ours) {
        (x, y) if x == y => 3 + pick_score(y), // draw
        ('A', 'B') |
        ('B', 'C') |
        ('C', 'A') => 6 + pick_score(alt_ours),
        _ => pick_score(alt_ours)
    }
}

fn losing_pick(pick: char) -> char
{
    match pick {
        'A' => 'C',
        'B' => 'A',
        'C' => 'B',
        _ => panic!()
    }
}

fn winning_pick(pick: char) -> char
{
    match pick {
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        _ => panic!()
    }
}

fn tactic_score(theirs: char, ours: char) -> u32
{
    match ours {
        'X' => pick_score(losing_pick(theirs)),
        'Y' => 3 + pick_score(theirs),
        'Z' => 6 + pick_score(winning_pick(theirs)),
        _ => panic!()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut score: u32 = 0;
    let mut tactic: u32 = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut ss = l.split_whitespace();
        match (ss.next(), ss.next()) {
            (Some(theirs), Some(ours)) => {
                let c1 = theirs.chars().nth(0).unwrap();
                let c2 = ours.chars().nth(0).unwrap();
                score += battle_score(c1, c2);
                tactic += tactic_score(c1, c2);
            }
            _ => {}
        }
    }
    println!("Part 1 answer: {}", score);
    println!("Part 2 answer: {}", tactic);
}
