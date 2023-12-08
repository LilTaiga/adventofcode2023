#![allow(unused, dead_code)]

use std::collections::HashMap;

type Place<'a> = &'a str;
type Directions<'a> = (&'a str, &'a str);

fn parse<'a>(
    input: &'a str,
) -> (
    impl Iterator<Item = char> + 'a,
    HashMap<Place<'a>, Directions<'a>>,
) {
    let mut lines_iter = input.lines();
    let directions_str = lines_iter.next().unwrap();
    lines_iter.next();

    let route = directions_str.chars().cycle();

    let places = lines_iter.map(|str| {
        let mut dir_iter = str.split_whitespace();

        let mut place = dir_iter.next().unwrap();
        dir_iter.next();
        let mut left = dir_iter
            .next()
            .and_then(|str| str.strip_prefix('('))
            .and_then(|str| str.strip_suffix(','))
            .unwrap();
        let mut right = dir_iter
            .next()
            .and_then(|str| str.strip_suffix(')'))
            .unwrap();

        (place, (left, right))
    });

    let mut map = HashMap::new();
    map.extend(places);

    (route, map)
}

pub fn part1(input: &str) -> usize {
    let (direction_iter, mut map) = parse(input);

    let mut current = "AAA";
    let mut steps = 0;

    for (i, c) in direction_iter.enumerate() {
        if current == "ZZZ" {
            break;
        }

        current = match c {
            'L' => map.entry(current).or_default().0,
            'R' => map.entry(current).or_default().1,
            _ => unreachable!(),
        };

        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let output = part1(input);
        assert_eq!(output, 2);
    }

    #[test]
    pub fn example1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let output = part1(input);
        assert_eq!(output, 6);
    }
}
