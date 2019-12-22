use anyhow::{anyhow, Result};
use std::str::FromStr;
use nom::{
    IResult,
    character::complete::{digit1, alpha1},
    bytes::complete::tag,
    combinator::{opt, map_res, recognize, map},
    sequence::{pair, separated_pair},
    multi::separated_list,
};
use std::collections::HashMap;
use num::Integer;

fn parse() -> Result<Vec<Recipe>> {
    std::fs::read_to_string("input")?
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect()
}

#[derive(Clone, Debug)]
struct Ingredient {
    chemical: String,
    quantity: i64,
}

#[derive(Clone, Debug)]
struct Recipe {
    output: Ingredient,
    inputs: Vec<Ingredient>,
}

fn parser(s: &str) -> IResult<&str, Recipe> {
    let int_parse = || map_res(recognize(pair(opt(tag("-")), digit1)), |s: &str| s.parse::<i64>());
    let str_parse = alpha1;
    let ing_parse = || map(separated_pair(int_parse(), tag(" "), str_parse), |(q, c)| Ingredient { chemical: c.to_owned(), quantity: q });
    let ing_seq_parse = separated_list(tag(", "), ing_parse());
    let recipe_parse = map(separated_pair(ing_seq_parse, tag(" => "), ing_parse()), |(i, o)| Recipe { output: o, inputs: i });
    recipe_parse(s)
}

impl FromStr for Recipe {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Recipe> {
        let (_, v) = parser(s).map_err(|_|anyhow!("failed to parse Recipe"))?;
        Ok(v)
    }
}

fn craft(ingredient: Ingredient, recipe_book: &HashMap<String, Recipe>, reserve: &mut HashMap<String, i64>) {
    if ingredient.chemical == "ORE" {
        return;
    }
    let recipe = &recipe_book[&ingredient.chemical];
    let runs = -(-ingredient.quantity).div_floor(&recipe.output.quantity);
    *reserve.entry(recipe.output.chemical.clone()).or_default() += runs*recipe.output.quantity;
    for i in &recipe.inputs {
        *reserve.entry(i.chemical.clone()).or_default() -= runs*i.quantity;
    }
}
fn balance(recipe_book: &HashMap<String, Recipe>, reserve: &mut HashMap<String, i64>) {
    loop {
        let mut to_balance = Vec::new();
        for chem in reserve.keys() {
            if chem != "ORE" && reserve[chem] < 0 {
                to_balance.push(chem.to_owned());
            }
        }
        if to_balance.len() == 0 {
            return;
        }
        for chem in to_balance {
            let quantity = -reserve[&chem];
            craft(Ingredient { chemical: chem, quantity}, recipe_book, reserve);
        }
    }
}
fn part1(recipes: Vec<Recipe>) -> Result<impl std::fmt::Display> {
    let recipe_book = {
        let mut recipe_book = HashMap::new();
        for r in recipes {
            recipe_book.insert(r.output.chemical.clone(), r);
        }
        recipe_book
    };
    let mut reserve = HashMap::new();
    let fuel = Ingredient { chemical: "FUEL".to_owned(), quantity: 1};
    craft(fuel, &recipe_book, &mut reserve);
    balance(&recipe_book, &mut reserve);
    Ok(-reserve["ORE"])
}
fn part2(recipes: Vec<Recipe>) -> Result<impl std::fmt::Display> {
    let recipe_book = {
        let mut recipe_book = HashMap::new();
        for r in recipes {
            recipe_book.insert(r.output.chemical.clone(), r);
        }
        recipe_book
    };
    let mut reserve = HashMap::new();
    reserve.insert("ORE".to_owned(), 1_000_000_000_000);
    let mut fuel_q = 0;
    let mut q = 1_877_913;
    loop {
        let fuel = Ingredient { chemical: "FUEL".to_owned(), quantity: q};
        craft(fuel, &recipe_book, &mut reserve);
        balance(&recipe_book, &mut reserve);
        if reserve["ORE"] < 0 {
            break;
        }
        fuel_q += q;
        if q > 1 {
            q = 1;
        }
    }
    Ok(fuel_q)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
