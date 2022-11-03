use slither_link_solver::board::{Board, Stat};

fn main() {
    let board = slither_link_solver::examples::example0();
    let mut board = Board::new(&board);
    println!("{}", board);
    println!("{:?}", board.check_consistency());
    let mut stat = Stat::default();
    let result = board.search(&mut stat);
    println!("result = {}", result);
    println!("{}", board);
    // stat = Stat { num_call: 6 }
    println!("stat = {:?}", stat);
}
