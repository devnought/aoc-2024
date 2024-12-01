use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use nom::{
    character::complete::{i64, space1},
    combinator::map,
    sequence::tuple,
    Finish, IResult,
};

fn main() -> anyhow::Result<()> {
    let part1 = part1()?;
    let part2 = part2()?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn part1() -> anyhow::Result<i64> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let (mut left, mut right): (Vec<_>, Vec<_>) = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line).finish().ok().map(|(_, data)| data))
        .unzip();

    left.sort();
    right.sort();

    let sum = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(sum)
}

fn part2() -> anyhow::Result<i64> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let (left, right): (Vec<_>, Vec<_>) = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line).finish().ok().map(|(_, data)| data))
        .unzip();

    let mut right_map: HashMap<i64, i64> = HashMap::new();
    for num in right.into_iter() {
        *right_map.entry(num).or_default() += 1;
    }

    let sum = left
        .into_iter()
        .map(|num| num * *right_map.entry(num).or_default())
        .sum();

    Ok(sum)
}

fn parser(input: &str) -> IResult<&str, (i64, i64)> {
    let parser = tuple((i64, space1, i64));
    map(parser, |(left, _, right)| (left, right))(input)
}
