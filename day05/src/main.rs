use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nom::{
    character::complete::{i64, space0},
    combinator::map,
    multi::many1,
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

    Ok(0)
}

fn part2() -> anyhow::Result<i64> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    Ok(0)
}

fn parser(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = tuple((i64, space0));
    let repeating = many1(parser);
    map(repeating, |r| r.into_iter().map(|(num, _)| num).collect())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
