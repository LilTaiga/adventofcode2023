mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() {
    let input = include_str!("../input.txt");

    let now = std::time::Instant::now();
    let output1 = part1(input);
    println!("Part 1 solution: {output1} -- Took {:?}", now.elapsed());

    let now = std::time::Instant::now();
    let output2 = part2(input);
    println!("Part 2 solution: {output2} -- Took {:?}", now.elapsed());
}
