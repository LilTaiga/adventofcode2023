#![allow(unused, dead_code)]

#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

fn parse(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning, numbers) = numbers.split_once(" | ").unwrap();

            let winning = winning
                .split(' ')
                .filter_map(|x| x.trim_start().parse().ok())
                .collect();

            let numbers = numbers
                .split(' ')
                .filter_map(|x| x.trim_start().parse().ok())
                .collect();

            Card {
                winning,
                numbers,
            }
        })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .fold(0, |acc, game| {
            let count = game.numbers.iter().filter(|num| game.winning.contains(num)).count();

            acc + match count {
                0 => 0,
                x => 2usize.pow(count as u32 - 1),
            }
        })
}
