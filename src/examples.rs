use crate::square::{parse_from_puzz_link, Square};

pub fn example0() -> Vec<Vec<Square>> {
    vec![vec![Square::Three; 2]]
}

pub fn example1() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/101601
    parse_from_puzz_link("https://puzz.link/p?slither/10/10/ld1080ddnbg2836dn1dn5380dgdnc0632daj")
        .unwrap()
}

pub fn example2() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/14223
    parse_from_puzz_link("https://puzz.link/p?slither/10/10/58287d517c11ch11bg222cg37d82226c73bg322dg12ch23a617b68388d").unwrap()
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Stat};

    use super::*;

    fn get_stat(board: Vec<Vec<Square>>) -> Stat {
        let mut board = Board::new(&board);
        let mut stat = Stat::default();
        let result = board.search(&mut stat, 100_000);
        assert!(result);
        stat
    }
    #[test]
    fn test_example0() {
        let stat = get_stat(example0());
        assert_eq!(stat.num_call, 6);
    }
    #[test]
    fn test_example1() {
        let stat = get_stat(example1());
        assert_eq!(stat.num_call, 2478);
    }
    #[test]
    fn test_example2() {
        let stat = get_stat(example2());
        assert_eq!(stat.num_call, 310);
    }
}
