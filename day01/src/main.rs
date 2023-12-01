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
    if slice.starts_with(b"one") {
        Some(1)
    } else if slice.starts_with(b"two") {
        Some(2)
    } else if slice.starts_with(b"three") {
        Some(3)
    } else if slice.starts_with(b"four") {
        Some(4)
    } else if slice.starts_with(b"five") {
        Some(5)
    } else if slice.starts_with(b"six") {
        Some(6)
    } else if slice.starts_with(b"seven") {
        Some(7)
    } else if slice.starts_with(b"eight") {
        Some(8)
    } else if slice.starts_with(b"nine") {
        Some(9)
    } else {
        None
    }
}

fn find_first_and_last_digits2(str: &[u8]) -> (usize, usize) {
    let mut first = None;
    let mut last = None;

    for i in 0..str.len() {
        let c = str[i];

        match c {
            d if d.is_ascii_digit() => {
                if first == None {
                    first = Some((d - b'0') as usize);
                } else {
                    last = Some((d - b'0') as usize);
                };
            },
            _ => {
                if let Some(d) = get_digit_from_str(&str[i..]) {
                    if first == None {
                        first = Some(d);
                    } else {
                        last = Some(d);
                    };
                }
            }
        }
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
