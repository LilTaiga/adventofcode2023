pub fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let mut game = line.split(':').skip(1);
            
            game.next().unwrap()
                .split(';').map(|game| {
                    let cubes = game.split(',');

                    let mut max_red = 0;
                    let mut max_green = 0;
                    let mut max_blue = 0;
                    
                    for cube in cubes {
                        let cube = cube.trim();
                        let (cube_num, cube_color) = cube.split_once(' ').unwrap();

                        let cube_num = cube_num.parse::<usize>().unwrap();
                        match (cube_num, cube_color) {
                            (n, "red") => max_red = usize::max(n, max_red),
                            (n, "green") => max_green = usize::max(n, max_green),
                            (n, "blue") => max_blue = usize::max(n, max_blue),
                            _ => {},
                        };
                    }

                    (max_red, max_green, max_blue)
                })
                .fold((0, 0, 0), |(r, g, b), (nr, ng, nb)|
                    (r.max(nr), g.max(ng), b.max(nb))
                )
        })
        .fold(0, |acc, (r, g, b)| acc + r * g * b)
}