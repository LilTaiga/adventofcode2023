#![allow(unused, dead_code)]

use std::{cmp::Ordering, collections::BTreeMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum Card {
    #[default]
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

struct Game {
    cards: [Card; 5],
    hand: HandType,
    bid: usize,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Game {}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => {}
            ord => return ord,
        }

        self.cards.cmp(&other.cards)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Game> + '_ {
    input
        .lines()
        .map(|str| str.split_once(' ').unwrap())
        .map(|(cards, bid)| {
            let (cards, jokers): ([Card; 5], usize) =
                cards
                    .chars()
                    .enumerate()
                    .fold(Default::default(), |mut acc, (i, c)| {
                        match c {
                            'J' => { acc.0[i] = Card::J; acc.1 += 1; },
                            '2' => acc.0[i] = Card::Two,
                            '3' => acc.0[i] = Card::Three,
                            '4' => acc.0[i] = Card::Four,
                            '5' => acc.0[i] = Card::Five,
                            '6' => acc.0[i] = Card::Six,
                            '7' => acc.0[i] = Card::Seven,
                            '8' => acc.0[i] = Card::Eight,
                            '9' => acc.0[i] = Card::Nine,
                            'T' => acc.0[i] = Card::T,
                            'Q' => acc.0[i] = Card::Q,
                            'K' => acc.0[i] = Card::K,
                            'A' => acc.0[i] = Card::A,
                            _ => unreachable!(),
                        }

                        acc
                    });

            let bid = bid.parse().unwrap();

            let mut map = BTreeMap::new();

            for card in cards.iter() {
                map.entry(card).and_modify(|e| *e += 1).or_insert(1);
            }

            let mut freq_dec = map.into_iter().collect::<Vec<_>>();
            freq_dec.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse());

            let hand = match (&freq_dec[..], jokers) {
                ([(_, 5), ..], _) => HandType::FiveKind,
                
                ([(_, 4), (_, 1), ..], 4) => HandType::FiveKind,
                ([(_, 4), (_, 1), ..], 1) => HandType::FiveKind,
                ([(_, 4), (_, 1), ..], 0) => HandType::FourKind,

                ([(_, 3), (_, 2), ..], 3) => HandType::FiveKind,
                ([(_, 3), (_, 2), ..], 2) => HandType::FiveKind,
                ([(_, 3), (_, 2), ..], 0) => HandType::FullHouse,
                ([(_, 3), (_, 1), (_, 1), ..], 3) => HandType::FourKind,
                ([(_, 3), (_, 1), (_, 1), ..], 1) => HandType::FourKind,
                ([(_, 3), (_, 1), (_, 1), ..], 0) => HandType::ThreeKind,

                ([(_, 2), (_, 2), (_, 1), ..], 2) => HandType::FourKind,
                ([(_, 2), (_, 2), (_, 1), ..], 1) => HandType::FullHouse,
                ([(_, 2), (_, 2), (_, 1), ..], 0) => HandType::TwoPair,
                ([(_, 2), (_, 1), (_, 1), (_, 1), ..], 2) => HandType::ThreeKind,
                ([(_, 2), (_, 1), (_, 1), (_, 1), ..], 1) => HandType::ThreeKind,
                ([(_, 2), (_, 1), (_, 1), (_, 1), ..], 0) => HandType::OnePair,

                ([(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)], 1) => HandType::OnePair, 
                ([(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)], 0) => HandType::HighCard, 

                _ => panic!("Impossible game!!!!!!!!")
            };

            Game { cards, hand, bid }
        })
}

pub fn part2(input: &str) -> usize {
    let mut games = parse(input).collect::<Vec<_>>();
    games.sort();

    games
        .iter()
        .enumerate()
        .map(|(rank, game)| (rank + 1) * game.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output = part2(input);
        assert_eq!(output, 5905);
    }
}
