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

pub fn part2(input: &str) -> usize {
    let (direction_iter, mut map) = parse(input);

    let mut currents = map
        .keys()
        .filter_map(|&str| if str.ends_with('A') { Some(str) } else { None })
        .collect::<Vec<_>>();
    let mut steps = 0;
    let mut steps_list = vec![None; currents.len()];

    for (i, c) in direction_iter.enumerate() {
         if steps_list.iter().all(|x| x.is_some()) {
            break;
         }
         
         steps += 1;
        for (j, current) in currents.iter_mut().enumerate() {
            *current = match c {
                'L' => map.entry(current).or_default().0,
                'R' => map.entry(current).or_default().1,
                _ => unreachable!(),
            };

            if current.ends_with('Z') {
                steps_list[j] = Some(steps);
            }
        }
    }

    steps_list.iter().skip(1).fold(steps_list[0].unwrap(), |acc, item| {
        let item = item.unwrap();

        lcd(acc, item)
    })
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let mut t = b;
        b = a % b;
        a = t
    }

    a
}

fn lcd(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let output = part2(input);
        assert_eq!(output, 6);
    }
}
