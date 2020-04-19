use std::env::args;
use taquin_rust::taquin::Taquin;
use taquin_rust::solver::Problem;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() {
    let args = args().collect::<Vec<String>>();
    let filename = if args.len() > 1 { args[1].clone() } else { String::from("./taquin_3x3.txt") };

    let taquins = read_taquins(filename).unwrap();
    println!("Found {} taquins to solve", taquins.len());
    for taquin in taquins {
        solve_taquin(taquin);
        println!();
    }
}

fn read_taquins(filename: String) -> Result<Vec<Taquin>, Error> {
    println!("Reading {}", filename);
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let taquins: Vec<Taquin> = buffer.lines()
        .map(|res| res.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| Taquin::from_str(3, line.as_str()))
        .collect();
    Ok(taquins)
}


fn solve_taquin(taquin: Taquin) {
    println!("Taquin to solve:\n{}", taquin);
    let solve = taquin.solve();
    let moves = solve.expect("Should be solvable");
    println!("Found a solution in {} steps", moves.len());
    print!("Moves: ");
    for m in moves {
        print!("{} ", m);
    }
    println!();
}

// fn main() {
//     let args = args().collect::<Vec<String>>();
//     let Config { size, tiles } = parse_config(&args);
//
//     let taquin = Taquin::from_str(size, &tiles);
//     solve_taquin(taquin);
// }
//
// struct Config {
//     size: u8,
//     tiles: String,
// }
//
// fn parse_config(args: &[String]) -> Config {
//     if args.len() <= 1 {
//         Config { size: 3, tiles: String::from("6,4,7,  8,5,0,  3,2,1") }
//     } else if args.len() == 2 {
//         let size = args[1].clone().parse::<u8>().unwrap_or(3);
//         Config { size, tiles: String::from("6,4,7,  8,5,0,  3,2,1") }
//     } else {
//         let size = args[1].clone().parse::<u8>().unwrap_or(3);
//         let tiles = args[2].clone();
//         Config { size, tiles }
//     }
// }
