#![allow(unused, dead_code)]

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn num_of_wins(&self) -> usize {
        (1..self.time)
            .map(|i| {
                let remaining_time = self.time - i;

                i * remaining_time
            })
            .filter(|&time| time > self.distance)
            .count()
    }
}

pub fn part2(input: &str) -> usize {
    let (time_str, dist_str) = input.split_once('\n').unwrap();

    let time_str = time_str
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();
    let dist_str = dist_str
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();

    Race {
        time: time_str.parse().unwrap(),
        distance: dist_str.parse().unwrap(),
    }
    .num_of_wins()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = part2(input);
        assert_eq!(output, 71503);
    }
}
