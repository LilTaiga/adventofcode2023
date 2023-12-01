mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() {
    let input = include_str!("../input.txt");

    let output1 = part1(input);
    dbg!(output1);

    let output2 = part2(input);
    dbg!(output2);
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
