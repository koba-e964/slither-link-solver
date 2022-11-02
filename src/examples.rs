use crate::square::{parse_from_puzz_link, Square};

pub fn example0() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/101601
    parse_from_puzz_link("https://puzz.link/p?slither/10/10/ld1080ddnbg2836dn1dn5380dgdnc0632daj")
        .unwrap()
}

pub fn example1() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/14223
    parse_from_puzz_link("https://puzz.link/p?slither/10/10/58287d517c11ch11bg222cg37d82226c73bg322dg12ch23a617b68388d").unwrap()
}
