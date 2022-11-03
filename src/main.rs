use std::io::stdin;

use slither_link_solver::{
    board::{Board, Stat},
    square::parse_from_puzz_link,
};

fn main() {
    for s in stdin().lines() {
        let s = s.unwrap();
        let board = if let Some(board) = parse_from_puzz_link(&s) {
            board
        } else {
            eprintln!("error while parsing: {}", s);
            continue;
        };
        let mut board = Board::new(&board);
        let mut stat = Stat::default();
        let limit = 10_000;
        let result = board.search(&mut stat, limit);
        println!("result = {}", result);
        println!("{}", board);
        println!("stat = {:?}", stat);
    }
}
