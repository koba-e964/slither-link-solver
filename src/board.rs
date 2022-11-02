use crate::square::Square;

// Up to 64
#[derive(Clone)]
pub struct Board<'sq> {
    init: &'sq [Vec<Square>],
    black_hori: Vec<u64>,
    black_vert: Vec<u64>,
    white_hori: Vec<u64>,
    white_vert: Vec<u64>,
}

impl<'sq> Board<'sq> {
    pub fn new(init: &'sq [Vec<Square>]) -> Self {
        let n = init.len();
        let m = init[0].len();
        assert!(m <= 63);
        assert!(init.iter().all(|v| v.len() == m));
        Self {
            init,
            black_hori: vec![0; n + 1],
            black_vert: vec![0; n],
            white_hori: vec![0; n + 1],
            white_vert: vec![0; n],
        }
    }
    pub fn finished(&self) -> bool {
        if self.check_consistency().is_err() {
            return false;
        }
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n + 1 {
            if (self.white_hori[i] | self.black_hori[i]) != (1 << m) - 1 {
                return false;
            }
        }
        for i in 0..n {
            if (self.white_vert[i] | self.black_vert[i]) != (1 << (m + 1)) - 1 {
                return false;
            }
        }
        true
    }
    pub fn check_consistency(&self) -> Result<(), ()> {
        Ok(())
    }
}

impl core::fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n + 1 {
            f.write_str("+")?;
            for j in 0..m {
                f.write_str(if (self.white_hori[i] & 1 << j) != 0 {
                    "-+"
                } else if (self.black_hori[i] & 1 << j) != 0 {
                    ".+"
                } else {
                    " +"
                })?;
            }
            writeln!(f)?;
            if i == n {
                break;
            }
            for j in 0..m + 1 {
                if j > 0 {
                    f.write_str(self.init[i][j - 1].as_str())?;
                }
                f.write_str(if (self.white_vert[i] & 1 << j) != 0 {
                    "|"
                } else if (self.black_vert[i] & 1 << j) != 0 {
                    "."
                } else {
                    " "
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
