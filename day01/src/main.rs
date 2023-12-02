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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let output = part1(input);
        assert_eq!(output, 142);
    }

    #[test]
    pub fn example2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let output = part2(input);
        assert_eq!(output, 281);
    }
}
