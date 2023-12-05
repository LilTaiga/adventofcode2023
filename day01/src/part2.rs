fn find_digit(slice: &[u8]) -> Option<usize> {
    match slice {
        [b'o', b'n', b'e', ..]              | [b'1', ..] => Some(1),
        [b't', b'w', b'o', ..]              | [b'2', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..]  | [b'3', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..]        | [b'4', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..]        | [b'5', ..] => Some(5),
        [b's', b'i', b'x', ..]              | [b'6', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..]  | [b'7', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..]  | [b'8', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..]        | [b'9', ..] => Some(9),
        _ => None,
    }
}

pub fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.as_bytes();
        let line_len = line.len();

        let mut first = 0;
        let mut last = 0;

        for i in 0..line_len {
            if let Some(num) = find_digit(&line[i..]) {
                first = num;
                break;
            }
        }

        for i in (0..line_len).rev() {
            if let Some(num) = find_digit(&line[i..line_len]) {
                last = num;
                break;
            }
        }

        sum += 10 * first + last;
    }

    sum
}