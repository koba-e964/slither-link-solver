#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Blank,
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl Square {
    pub fn new_number(a: u8) -> Option<Self> {
        match a {
            0 => Some(Square::Zero),
            1 => Some(Square::One),
            2 => Some(Square::Two),
            3 => Some(Square::Three),
            4 => Some(Square::Four),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Square::Blank => " ",
            Square::Zero => "0",
            Square::One => "1",
            Square::Two => "2",
            Square::Three => "3",
            Square::Four => "4",
        }
    }

    pub fn get_number(self) -> Option<u8> {
        Some(match self {
            Square::Blank => return None,
            Square::Zero => 0,
            Square::One => 1,
            Square::Two => 2,
            Square::Three => 3,
            Square::Four => 4,
        })
    }
}

/// Parses a URL like https://puzz.link/p?slither_link/9/9/.2zzzy
/// or https://puzz.link/p?slither_link_edit/9/9/.2zzzy.
pub fn parse_from_puzz_link(s: &str) -> Option<Vec<Vec<Square>>> {
    let s = if let Some(t) = s.strip_prefix("https://puzz.link/p?slither") {
        t
    } else {
        return None;
    };
    let s = if let Some(t) = s.strip_prefix("/") {
        t
    } else if let Some(t) = s.strip_prefix("_edit/") {
        t
    } else {
        return None;
    };
    let split: Vec<_> = s.split("/").collect();
    if split.len() != 3 {
        return None;
    }
    let m = split[0].parse::<usize>().ok()?;
    let n = split[1].parse::<usize>().ok()?;
    if n * m >= 10_000 || split[2].len() >= 10_000 {
        return None;
    }
    let mut data = vec![];
    for c in split[2].chars() {
        if c == '.' {
            // '?' is treated as an empty square.
            data.push(Square::Blank);
            continue;
        }
        if c >= 'g' && c <= 'z' {
            let len = (c as u8 - b'f') as usize;
            data.extend_from_slice(&vec![Square::Blank; len]);
            continue;
        }
        if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'e') {
            let dig = if c <= '9' {
                c as u8 - b'0'
            } else {
                c as u8 - b'a' + 10
            };
            data.push(Square::new_number(dig % 5)?);
            for _ in 0..dig / 5 {
                data.push(Square::Blank);
            }
            continue;
        }
        return None;
    }
    data.truncate(n * m);
    if data.len() != n * m {
        return None;
    }
    let mut ret = vec![vec![Square::Blank; m]; n];
    for i in 0..n {
        ret[i].copy_from_slice(&data[i * m..i * m + m]);
    }
    Some(ret)
}
