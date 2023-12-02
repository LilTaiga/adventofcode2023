mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() {
    let input = include_str!("../input.txt");

    let now = std::time::Instant::now();
    let output1 = part1(input);
    let elapsed = now.elapsed();
    println!("Part 1 solution: {output1} -- Took {elapsed:?}");

    let now = std::time::Instant::now();
    let output2 = part2(input);
    let elapsed = now.elapsed();
    println!("Part 2 solution: {output2} -- Took {elapsed:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = part1(input);
        assert_eq!(output, 8);
    }

    #[test]
    pub fn example2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = part2(input);
        assert_eq!(output, 2286);
    }
}
