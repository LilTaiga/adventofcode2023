#![allow(unused, dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    numbers: Vec<usize>,
    win_count: usize,
}

fn parse(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning, numbers) = numbers.split_once(" | ").unwrap();

            let winning: Vec<_> = winning
                .split(' ')
                .filter_map(|x| x.trim_start().parse::<usize>().ok())
                .collect();

            let numbers: Vec<_> = numbers
                .split(' ')
                .filter_map(|x| x.trim_start().parse::<usize>().ok())
                .collect();
            
            let win_count = numbers.iter().filter(|num| winning.contains(num)).count();

            Card {
                winning,
                numbers,
                win_count,
            }
        })
}

fn count_wins(cards: &[Card]) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::with_capacity(200);
    map.extend((0..cards.len()).map(|i| (i, 1)));

    for (i, card) in cards.iter().enumerate() {
        let win_count = card.win_count;
        let card_count = map[&i];
        
        for j in 1..=win_count {
            if i + j >= cards.len() {
                break;
            }

            map.entry(i + j)
                .and_modify(|x| *x += card_count);
        }
    }

    map.values().sum()
}

pub fn part2(input: &str) -> usize {
    let cards: Vec<_> = parse(input).collect();
    
    count_wins(&cards)
}
