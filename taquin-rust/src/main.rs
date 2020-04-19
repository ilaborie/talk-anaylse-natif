use std::env::args;
use taquin_rust::taquin::Taquin;
use taquin_rust::solver::Problem;

fn main() {
    let args = args().collect::<Vec<String>>();
    let Config { size, tiles } = parse_config(&args);

    let taquin = Taquin::from_str(size, &tiles);
    println!("Taquin to solve:\n{}", taquin);

    let solve = taquin.solve();
    let moves = solve.unwrap();
    println!("Found a solution in {} steps", moves.len());
    println!("Moves:");
    for (i, &m) in moves.iter().enumerate() {
        println!("#{}: {}", i, m);
    }
}

struct Config {
    size: u8,
    tiles: String,
}

fn parse_config(args: &[String]) -> Config {
    if args.len() <= 1 {
        Config { size: 3, tiles: String::from("5, 0, 3,  8, 1, 2,  4, 7, 6") }
    } else if args.len() == 2 {
        let size = args[1].clone().parse::<u8>().unwrap_or(3);
        Config { size, tiles: String::from("5, 0, 3,  8, 1, 2,  4, 7, 6") }
    } else {
        let size = args[1].clone().parse::<u8>().unwrap_or(3);
        let tiles = args[2].clone();
        Config { size, tiles }
    }
}


