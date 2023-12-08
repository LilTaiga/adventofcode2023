#![allow(unused, dead_code)]

use std::{cmp::Ordering, collections::BTreeMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
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
            let cards: [Card; 5] =
                cards
                    .chars()
                    .enumerate()
                    .fold(Default::default(), |mut acc, (i, c)| {
                        match c {
                            '2' => acc[i] = Card::Two,
                            '3' => acc[i] = Card::Three,
                            '4' => acc[i] = Card::Four,
                            '5' => acc[i] = Card::Five,
                            '6' => acc[i] = Card::Six,
                            '7' => acc[i] = Card::Seven,
                            '8' => acc[i] = Card::Eight,
                            '9' => acc[i] = Card::Nine,
                            'T' => acc[i] = Card::T,
                            'J' => acc[i] = Card::J,
                            'Q' => acc[i] = Card::Q,
                            'K' => acc[i] = Card::K,
                            'A' => acc[i] = Card::A,
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

            let hand = match &freq_dec[..] {
                [(_, 5), ..] => HandType::FiveKind,
                [(_, 4), ..] => HandType::FourKind,
                [(_, 3), (_, 2), ..] => HandType::FullHouse,
                [(_, 3), ..] => HandType::ThreeKind,
                [(_, 2), (_, 2), ..] => HandType::TwoPair,
                [(_, 2), ..] => HandType::OnePair,
                _ => HandType::HighCard,
            };

            Game { cards, hand, bid }
        })
}

pub fn part1(input: &str) -> usize {
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
    pub fn example1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output = part1(input);
        assert_eq!(output, 6440);
    }
}
