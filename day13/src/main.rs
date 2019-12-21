use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use rltk::{Rltk, GameState, Console, VirtualKeyCode};

fn parse() -> Result<Vec<i64>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn part1(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v);
    let mut map = HashMap::new();
    loop {
        if let Some(x) = vm.run_until_output()? {
            let y = vm.run_until_output()?.ok_or_else(||anyhow!("no y coord"))?;
            let t = vm.run_until_output()?.ok_or_else(||anyhow!("no tile"))?;
            map.insert((x,y), t);
        } else {
            break;
        }
    }
    let count = map.values().filter(|&&t| t==2).count();
    Ok(count)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}
impl Tile {
    fn parse(i: i64) -> Result<Tile> {
        match i {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(anyhow!("cannot parse tile")),
        }
    }
}
struct Game {
    vm: intcode::Vm,
    score: i64,
    map: Vec<Tile>,
    ball: i64,
    paddle: i64,
}

const WIDTH: usize = 44;
const HEIGHT: usize = 24;
impl Game {
    fn new(mut mem: Vec<i64>) -> Game {
        mem[0] = 2;
        let mut map = Vec::new();
        map.resize(WIDTH*HEIGHT, Tile::Empty);
        Game {
            vm: intcode::Vm::new(mem),
            map,
            score: 0,
            ball: 0,
            paddle: 0,
        }
    }
    fn map_at(&mut self, x: i64, y: i64) -> Result<&mut Tile> {
        if x < 0 || x >= (WIDTH as i64) || y < 0 || y >= (HEIGHT as i64) {
            Err(anyhow!("out of map access"))
        } else {
            Ok(&mut self.map[x as usize + WIDTH*(y as usize)])
        }
    }
    fn update(&mut self) -> Result<bool> {
        if !self.vm.is_running() {
            return Ok(false);
        }
        let ball = self.ball;
        let paddle = self.paddle;
        let input = || {
            if ball == paddle {
                0
            } else if ball < paddle {
                -1
            } else {
                1
            }
        };
        if let Some(x) = self.vm.run_until_output_with_input(input)? {
            let y = self.vm.run_until_output_with_input(input)?.ok_or_else(||anyhow!("no y coord"))?;
            let t = self.vm.run_until_output_with_input(input)?.ok_or_else(||anyhow!("no tile"))?;

            if x == -1 {
                self.score = t;
            } else {
                let tile = Tile::parse(t)?;
                *self.map_at(x, y)? = tile;
                if tile == Tile::Ball {
                    self.ball = x;
                } else if tile == Tile::Paddle {
                    self.paddle = x;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut Rltk) {
        if !self.update().expect("update error") {
            //ctx.quit();
            return;
        }
        ctx.cls();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let ch = match *self.map_at(x as i64, y as i64).unwrap() {
                    Tile::Empty => {
                        "."
                    },
                    Tile::Block => {
                        "O"
                    },
                    Tile::Wall => {
                        "#"
                    },
                    Tile::Paddle => {
                        "="
                    },
                    Tile::Ball => {
                        "o"
                    },
                };
                ctx.print(x as i32, y as i32, ch);
            }
        }
        ctx.print(0, (HEIGHT+1) as i32, &format!("score: {}", self.score));
    }
}
fn part2(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let game = Game::new(v);
    let ctx = Rltk::init_simple8x8(WIDTH as u32, (HEIGHT+2) as u32, "breakout", "resources");
    rltk::main_loop(ctx, game);
    Ok(1)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
