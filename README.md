# Advent of Code 2023
This is my first attempt at AoC. \
This repository holds my attempted solutions in Rust to the problems of [Advent of Code 2023](https://adventofcode.com/2023). \
Feel free to look around.

## Running the solutions
From this folder, execute the command \
`cargo run --bin dayX` \
to run the solution for day `X`.

You can also execute \
`cargo test --bin dayX` \
to verify the tests for that day.

## My benchmarks
I execute `cargo run --bin dayX --release` 5 times and take the average for each day. 

|Days   | Part 1   | Part 2   |
|------ |----------|----------|
|Day 01 | 197.94µs | 125.78µs |
|Day 02 | 84.58µs  | 86.86µs  |
|Day 03 |          |          |
|Day 04 | 388.12µs | 926.9µs  |