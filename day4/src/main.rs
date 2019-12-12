use anyhow::anyhow;
use anyhow::Result;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u8)]
enum Digit {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
}

const DIGITS: [Digit; 10] = [
    Digit::D0,
    Digit::D1,
    Digit::D2,
    Digit::D3,
    Digit::D4,
    Digit::D5,
    Digit::D6,
    Digit::D7,
    Digit::D8,
    Digit::D9,
];
impl Digit {
    fn next(self) -> Option<Digit> {
        DIGITS.get(self as usize + 1).map(|d| *d)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Password {
    digits: [Digit; 6],
}

impl Password {
    fn check_double(&self) -> bool {
        self.digits.windows(2).any(|w| w[0] == w[1])
    }
    fn check_double_strict(&self) -> bool {
        let mut i = 0;
        'l: loop {
            for j in i+1..6 {
                if self.digits[i] != self.digits[j] {
                    if j - i == 2 {
                        return true;
                    }
                    i = j;
                    continue 'l;
                } else if i == 4 {
                    return true;
                }
            }
            return false;
        }
    }
    fn check_increase(&self) -> bool {
        self.digits.windows(2).all(|w| w[0] <= w[1])
    }
    fn check(&self) -> bool {
        self.check_double() && self.check_increase()
    }
    fn check_strict(&self) -> bool {
        self.check_double_strict() && self.check_increase()
    }
    fn next(mut self) -> Option<Password> {
        for d in self.digits.iter_mut().rev() {
            if let Some(n) = d.next() {
                *d = n;
                return Some(self)
            } else {
                *d = Digit::D0;
            }
        }
        None
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}",
            self.digits[0] as u8,
            self.digits[1] as u8,
            self.digits[2] as u8,
            self.digits[3] as u8,
            self.digits[4] as u8,
            self.digits[5] as u8,
        )
    }
}

impl std::str::FromStr for Password {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Password, Self::Err> {
        if s.len() != 6 {
            return Err(anyhow!("Password is not 6 digits"));
        }
        let mut digits = [Digit::D0; 6];
        for (i, c) in s.chars().enumerate() {
            digits[i] = Digit::try_from(
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("Password contains non digit"))? as u8,
            )
            .unwrap();
        }
        Ok(Password { digits })
    }
}

fn parse() -> Result<(Password, Password)> {
    let f = std::fs::read_to_string("input")?;
    let mut it = f.trim().split('-').map(std::str::FromStr::from_str);

    let min = it.next().ok_or_else(|| anyhow!("parsing error"))??;
    let max = it.next().ok_or_else(|| anyhow!("parsing error"))??;
    Ok((min, max))
}

fn part1((min, max): (Password, Password)) -> Result<impl std::fmt::Display> {
    let mut count = 0;
    let mut cur = min;
    while let Some(c) = cur.next() {
        cur = c;
        if cur > max {
            break;
        }
        count += cur.check() as u32;
    }
    Ok(count)
}

fn part2((min, max): (Password, Password)) -> Result<impl std::fmt::Display> {
    let mut count = 0;
    let mut cur = min;
    while let Some(c) = cur.next() {
        cur = c;
        if cur > max {
            break;
        }
        count += cur.check_strict() as u32;
    }
    Ok(count)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v)?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
