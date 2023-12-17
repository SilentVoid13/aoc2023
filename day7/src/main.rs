use std::time::Instant;

use color_eyre::eyre::{ContextCompat, Result};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    typ: HandType,
    cards: [u8; 5],
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn parse_hand(s: &str, with_joker: bool) -> Result<Hand> {
    let (str_cards, bid) = s.split_once(" ").wrap_err("invalid hand")?;
    let bid = bid.parse::<usize>()?;
    let mut cards = [0u8; 5];
    for (i, c) in str_cards.chars().enumerate() {
        let v = match c {
            'A' => 15,
            'J' => if with_joker { 0 } else { 12 },
            'T' => 11,
            'Q' => 13,
            'K' => 14,
            '1'..='9' => c as u8 - '0' as u8,
            _ => unreachable!(),
        };
        cards[i] = v;
    }
    let mut count = vec![0_u8; 16];
    for c in cards.iter() {
        count[*c as usize] += 1;
    }
    if with_joker {
        let jk_count = count[0];
        count.remove(0);
        count.sort();
        count.reverse();
        count[0] += jk_count;
    } else {
        count.sort();
        count.reverse();
    }

    let typ = match count[..2] {
        [1, ..] => HandType::High,
        [2, 1] => HandType::OnePair,
        [2, 2] => HandType::TwoPair,
        [3, 1] => HandType::Three,
        [3, 2] => HandType::Full,
        [4, ..] => HandType::Four,
        [5, ..] => HandType::Five,
        _ => unreachable!(),
    };
    let hand = Hand { typ, cards, bid };
    Ok(hand)
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.typ != other.typ {
            return Some(self.typ.cmp(&other.typ));
        }
        Some(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut hands: Vec<Hand> = input.lines().filter_map(|l| parse_hand(l, false).ok()).collect();
    hands.sort();
    let mut win = 0;
    for (i, hand) in hands.iter().enumerate() {
        win += (i + 1) * hand.bid;
    }
    Ok(win)
}

fn part2(input: &str) -> Result<usize> {
    let mut hands: Vec<Hand> = input.lines().filter_map(|l| parse_hand(l, true).ok()).collect();
    hands.sort();
    let mut win = 0;
    for (i, hand) in hands.iter().enumerate() {
        win += (i + 1) * hand.bid;
    }
    Ok(win)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input");

    let instant = Instant::now();
    let res = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", res, time);

    let instant = Instant::now();
    let res = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", res, time);

    Ok(())
}
