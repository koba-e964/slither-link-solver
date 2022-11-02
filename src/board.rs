use crate::square::Square;

#[derive(Debug, Clone, Default)]
pub struct Stat {
    pub(crate) num_call: u64,
}

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
        self.check_vertices()?;
        self.check_faces()?;
        self.check_global()?;
        Ok(())
    }
    pub fn check_vertices(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n + 1 {
            for j in 0..m + 1 {
                let mut black_limit = 0;
                let mut black = 0;
                if i > 0 {
                    if (self.black_vert[i - 1] & 1 << j) != 0 {
                        black += 1;
                    }
                    if (self.white_vert[i - 1] & 1 << j) == 0 {
                        black_limit += 1;
                    }
                }
                if j > 0 {
                    if (self.black_hori[i] & 1 << (j - 1)) != 0 {
                        black += 1;
                    }
                    if (self.white_hori[i] & 1 << (j - 1)) == 0 {
                        black_limit += 1;
                    }
                }
                if i < n {
                    if (self.black_vert[i] & 1 << j) != 0 {
                        black += 1;
                    }
                    if (self.white_vert[i] & 1 << j) == 0 {
                        black_limit += 1;
                    }
                }
                if j < m {
                    if (self.black_hori[i] & 1 << j) != 0 {
                        black += 1;
                    }
                    if (self.white_hori[i] & 1 << j) == 0 {
                        black_limit += 1;
                    }
                }
                if [0, 2].iter().all(|&x| x < black || x > black_limit) {
                    return Err(());
                }
            }
        }
        Ok(())
    }

    pub fn check_faces(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            for j in 0..m {
                let white = (self.white_hori[i] & 1 << j).count_ones()
                    + (self.white_hori[i + 1] & 1 << j).count_ones()
                    + (self.white_vert[i] & 3 << j).count_ones();
                let black = (self.black_hori[i] & 1 << j).count_ones()
                    + (self.black_hori[i + 1] & 1 << j).count_ones()
                    + (self.black_vert[i] & 3 << j).count_ones();
                let black_limit = 4 - white;
                if let Some(number) = self.init[i][j].get_number() {
                    if u32::from(number) < black || black_limit < u32::from(number) {
                        return Err(());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn check_global(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn search(&mut self, stat: &mut Stat) -> bool {
        stat.num_call += 1;
        if self.finished() {
            return true;
        }
        if self.check_consistency().is_err() {
            return false;
        }
        let n = self.init.len();
        let m = self.init[0].len();
        // very naive search
        for i in 0..n + 1 {
            let white = self.white_hori[i];
            let black = self.black_hori[i];
            let occupied = white | black;
            for j in 0..m {
                if (occupied & 1 << j) != 0 {
                    continue;
                }
                self.white_hori[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.white_hori[i] ^= 1 << j;
                self.black_hori[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.black_hori[i] ^= 1 << j;
                return false;
            }
        }
        for i in 0..n {
            let white = self.white_vert[i];
            let black = self.black_vert[i];
            let occupied = white | black;
            for j in 0..m + 1 {
                if (occupied & 1 << j) != 0 {
                    continue;
                }
                self.white_vert[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.white_vert[i] ^= 1 << j;
                self.black_vert[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.black_vert[i] ^= 1 << j;
                return false;
            }
        }
        false
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
                    ".+"
                } else if (self.black_hori[i] & 1 << j) != 0 {
                    "-+"
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
                    "."
                } else if (self.black_vert[i] & 1 << j) != 0 {
                    "|"
                } else {
                    " "
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
