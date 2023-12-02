#[allow(dead_code)]
struct Game {
    id: usize,
    cubes: (usize, usize, usize),
}

fn parse(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|str| {
        let (id, games) = str.split_once(": ").unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let id: usize = id.parse().unwrap();

        let cubes = games
            .split("; ")
            .map(|game| {
                game.split(", ").fold((0, 0, 0), |(r, g, b), cube| {
                    let (num, color) = cube.split_once(' ').unwrap();
                    let num: usize = num.parse().unwrap();

                    match (num, color) {
                        (n, "red") => (r.max(n), g, b),
                        (n, "green") => (r, g.max(n), b),
                        (n, "blue") => (r, g, b.max(n)),
                        _ => panic!("Something went very wrong!"),
                    }
                })
            })
            .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                (max_r.max(r), max_g.max(g), max_b.max(b))
            });

        Game { id, cubes }
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter_map(|game| {
            if game.cubes.0 <= 12 && game.cubes.1 <= 13 && game.cubes.2 <= 14 {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}
