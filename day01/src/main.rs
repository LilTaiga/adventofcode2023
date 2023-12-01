fn find_first_and_last_digits(str: &[u8]) -> (usize, usize) {
    let mut digits_iter = str.iter().filter(|&&c| c.is_ascii_digit());

    let first = (digits_iter.next().unwrap() - b'0') as usize;
    let mut last = None;
    
    for c in digits_iter {
        let d = (c - b'0') as usize;

        last = Some(d);
    }

    match last {
        Some(d) => (first, d),
        None => (first, first),
    }
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    
    for line in input.lines() {
        let (first, last) = find_first_and_last_digits(line.as_bytes());

        sum += first * 10 + last;
    }

    sum
}

fn get_digit_from_str(slice: &[u8]) -> Option<usize> {
    match slice {
        [b'o', b'n', b'e', ..] => Some(1),
        [b't', b'w', b'o', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..] => Some(5),
        [b's', b'i', b'x', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..] => Some(9),
        _ => None
    }
}

fn find_first_and_last_digits2(str: &[u8]) -> (usize, usize) {
    let mut first = None;
    let mut last = None;

    let mut i = 0;
    while i < str.len() {
        let c = str[i];

        if c.is_ascii_digit() {
            if first == None {
                first = Some((c - b'0') as usize);
            } else {
                last = Some((c - b'0') as usize);
            }
        } else {
            if let Some(d) = get_digit_from_str(&str[i..]) {
                if first == None {
                    first = Some(d);
                } else {
                    last = Some(d);
                }
            }
        }

        i += 1;
    }

    match last {
        Some(d) => (first.unwrap(), d),
        None => (first.unwrap(), first.unwrap()),
    }
}

fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let (first, last) = find_first_and_last_digits2(line.as_bytes());

        sum += first * 10 + last;
    }

    sum
}

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
