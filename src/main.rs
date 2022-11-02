use slither_link_solver::board::Board;

fn main() {
    let board = slither_link_solver::examples::example0();
    let board = Board::new(&board);
    println!("{}", board);
}
