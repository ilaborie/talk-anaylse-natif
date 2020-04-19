use taquin_rust::taquin::Taquin;
use taquin_rust::solver::Problem;

#[test]
fn already_solved() {
    let taquin = Taquin::new(3);
    let result = taquin.solve();
    assert_eq!(result, Ok(vec![]))
}


#[test]
fn solve_one_move() {
    let mut taquin = Taquin::new(3);
    taquin.shuffle(1);
    let result = taquin.solve().unwrap();
    assert_eq!(result.len(), 1)
}

#[test]
fn solve_few_moves() {
    let mut taquin = Taquin::new(3);
    taquin.shuffle(5);
    let result = taquin.solve().expect("Oops!");
    println!("Solve in {} moves", result.len());
    assert_eq!(result.len(), 5);
    // assert!(result.len() <= 5)
}

#[test]
fn solve_a_3x3() {
    let taquin = Taquin::from_str(3, "6,4,7,  8,5,0,  3,2,1");
    let result = taquin.solve().expect("Oops!");
    println!("Solve in {} moves", result.len());
    // assert_eq!(result.len(), 31);

    let end= result.iter()
        .fold(taquin, |t, m| t.move_hole(*m));

    assert!(end.is_solved())
}
