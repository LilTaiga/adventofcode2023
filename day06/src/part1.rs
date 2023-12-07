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

pub fn part1(input: &str) -> usize {
    let (time_str, dist_str) = input.split_once('\n').unwrap();

    time_str
        .split_ascii_whitespace()
        .zip(dist_str.split_ascii_whitespace())
        .skip(1)
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .map(|race| race.num_of_wins())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = part1(input);
        assert_eq!(output, 288);
    }
}
