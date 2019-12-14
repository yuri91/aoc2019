use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct Tree {
    nodes: HashMap<String, Vec<String>>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: HashMap::new(),
        }
    }
    fn add(&mut self, from: String, to: String) {
        self.nodes.entry(to.clone()).or_insert_with(Vec::new);
        let n = self.nodes.entry(from).or_insert_with(Vec::new);
        n.push(to);
    }
    fn level_sum(&self, root: &str, level: u32) -> u32 {
        let com = self.nodes.get(root).expect("no tree root");
        let mut sum = 0;
        for child in com {
            sum += self.level_sum(child, level + 1);
        }
        sum + level
    }
    fn get_path(&self, from: &str, to: &str) -> Option<Vec<String>>{
        if from == to {
            return Some(Vec::new());
        }
        let com = self.nodes.get(from).expect("no tree root");
        for child in com {
            if let Some(mut path) = self.get_path(child, to) {
                path.push(from.to_owned());
                return Some(path);
            }
        }
        None
    }
}

fn parse() -> Result<Vec<(String, String)>> {
    std::fs::read_to_string("input")?
        .trim()
        .split('\n')
        .map(|l| {
            let mut it = l.split(')');
            Ok((
                it.next().ok_or_else(|| anyhow!("cannot parse orbit"))?.to_owned(),
                it.next().ok_or_else(|| anyhow!("cannot parse orbit"))?.to_owned(),
            ))
        })
        .collect()
}

fn part1(v: Vec<(String, String)>) -> Result<impl std::fmt::Display> {
    let mut tree = Tree::new();
    for i in v {
        tree.add(i.0, i.1);
    }

    Ok(tree.level_sum("COM", 0))
}

fn part2(v: Vec<(String, String)>) -> Result<impl std::fmt::Display> {
    let mut tree = Tree::new();
    for i in v {
        tree.add(i.0, i.1);
    }
    let mut path1 = tree.get_path("COM", "YOU").ok_or_else(|| anyhow!("no path from COM to YOU"))?.into_iter().rev();
    let mut path2 = tree.get_path("COM", "SAN").ok_or_else(|| anyhow!("no path from COM to SAN"))?.into_iter().rev();

    let mut count = 0;
    loop {
        let n1 = path1.next();
        let n2 = path2.next();

        if n1.is_none() || n2.is_none() || n1 != n2 {
            count += n1.is_some() as i32 + n2.is_some() as i32;
            break;
        }
    }
    for _ in path1.chain(path2) {
        count += 1;
    }

    Ok(count)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
