use anyhow::{anyhow, Result};
use std::collections::HashSet;

fn parse() -> Result<Vec<u8>> {
    std::fs::read_to_string("input")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).ok_or_else(|| anyhow!("not a digit")))
        .collect()
}

struct Image {
    data: Vec<u8>,
    w: usize,
    h: usize,
}

impl Image {
    fn new(data: Vec<u8>, w: usize, h: usize) -> Image {
        Image {
            data,
            w,
            h,
        }
    }

    fn size(&self) -> usize {
        self.h * self.w
    }
    fn num_layers(&self) -> usize {
        self.data.len() / self.size()
    }
    fn layer(&self, n: usize) -> &[u8] {
        &self.data[n*self.size()..(n+1)*self.size()]
    }
    fn layers(&self) -> impl Iterator<Item=&[u8]> + '_ {
        (0..self.num_layers()).map(move |n| self.layer(n))
    }
    fn get_pixel(&self, x: usize, y: usize) -> u8 {
        for l in self.layers() {
            let p = l[x + y*self.w];
            if p != 2 {
                return p;
            }
        }
        return 0;
    }
}

fn part1(v: Vec<u8>) -> Result<impl std::fmt::Display> {
    let img = Image::new(v, 25, 6);
    let l = img.layers().min_by_key(|l| {
        l.iter().filter(|&&i| i == 0).count()
    }).unwrap();
    let ones = l.iter().filter(|&&i| i == 1).count();
    let twos = l.iter().filter(|&&i| i == 2).count();
    Ok(ones * twos)
}

fn part2(v: Vec<u8>) -> Result<impl std::fmt::Display> {
    let img = Image::new(v, 25, 6);
    let mut res = String::new();
    res.push('\n');
    for j in 0..6 {
        for i in 0..25 {
            if img.get_pixel(i, j) == 0 {
                res.push(' ');
            } else {
                res.push('█');
            }
        }
        res.push('\n');
    }
    Ok(res)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
