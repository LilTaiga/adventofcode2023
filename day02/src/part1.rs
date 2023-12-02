pub fn part1(input: &str) -> usize {
    let possible_games = input.lines()
        .map(|line| {
            let mut game = line.split(':');
            
            let game_id = game.next().unwrap()
                .split(' ').skip(1).next().unwrap()
                .parse::<usize>().unwrap();

            
            let is_possible = game.next().unwrap()
                .split(';').map(|game| {
                    let cubes = game.split(',');
                    
                    for cube in cubes {
                        let cube = cube.trim();
                        let (cube_num, cube_color) = cube.split_once(' ').unwrap();

                        let cube_num = cube_num.parse::<usize>().unwrap();
                        match (cube_num, cube_color) {
                            (n, "red") if n <= 12 => continue,
                            (n, "green") if n <= 13 => continue,
                            (n, "blue") if n <= 14 => continue,
                            _ => return false,
                        }
                    }

                    true
                }).all(|x| x == true);

            (game_id, is_possible)
        });

    possible_games.filter_map(|(id, bool)| if bool { Some(id) } else { None }).sum()
}