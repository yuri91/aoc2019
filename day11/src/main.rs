use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;

fn parse() -> Result<Vec<i64>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

const DIRS: [(i64, i64);4] = [
    (0,1),
    (1,0),
    (0,-1),
    (-1,0),
];

fn paint_area(vm: &mut intcode::Vm, area: &mut HashMap<(i64, i64), i64>) -> Result<()>{
    let mut cur_pos = (0, 0);
    let mut cur_dir = 0;
    while vm.is_running() {
        let v = area.entry(cur_pos).or_insert(0);
        vm.add_inputs(&[*v]);
        let color = if let Some(c) = vm.run_until_output()? {
            c
        } else {
            break;
        };
        area.insert(cur_pos, color);
        let new_dir = vm.run_until_output()?.ok_or_else(||anyhow!("missing dir"))?;
        match new_dir {
            0 => {
                cur_dir = (cur_dir + DIRS.len() - 1) % DIRS.len();
            },
            1 => {
                cur_dir = (cur_dir + DIRS.len() + 1) % DIRS.len();
            },
            _ => {
                return Err(anyhow!("invalid direction received"));
            },
        }
        let dir = DIRS[cur_dir];
        cur_pos = (cur_pos.0+dir.0, cur_pos.1+dir.1);
    }
    Ok(())
}
fn part1(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v);
    let mut area = HashMap::new();
    paint_area(&mut vm, &mut area)?;
    Ok(area.len())
}

fn part2(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v);
    let mut area = HashMap::new();
    area.insert((0,0), 1);
    paint_area(&mut vm, &mut area)?;
    let left = *area.keys().map(|(x,_)|x).min().unwrap();
    let bottom = *area.keys().map(|(_,y)|y).min().unwrap();
    let right = *area.keys().map(|(x,_)|x).max().unwrap();
    let top = *area.keys().map(|(_,y)|y).max().unwrap();
    let mut canvas = Vec::new();
    canvas.resize_with((top-bottom+1) as usize, Vec::new);
    for row in &mut canvas {
        row.resize((right-left+1) as usize,' ');
    }
    for ((x,y),v) in area {
        let x = (x - left) as usize;
        let y = (y - bottom) as usize;
        canvas[y][x] = if v == 0 { ' ' } else { '█' };
    }

    let mut res = "\n".to_owned();
    for row in canvas.into_iter().rev() {
        res.extend(row);
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
