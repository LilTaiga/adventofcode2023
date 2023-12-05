fn find_digit(slice: char) -> Option<usize> {
    match slice {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

pub fn part1(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let first = line.chars().find_map(find_digit).unwrap();
        let last = line.chars().rev().find_map(find_digit).unwrap();

        sum += format!("{first}{last}").parse::<usize>().unwrap();
    }

    sum
}
