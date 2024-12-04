use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, i64},
    combinator::map,
    multi::many_till,
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
    let raw_input = fs::read_to_string("./input.txt")?;
    let mut input = raw_input.as_str();
    let mut sum = 0;

    while let Ok((remaining, instruction)) = combo_parser(input).finish() {
        if let Instruction::Mul(left, right) = instruction {
            sum += left * right;
        };

        input = remaining;
    }

    Ok(sum)
}

fn part2() -> anyhow::Result<i64> {
    let raw_input = fs::read_to_string("./input.txt")?;
    let mut input = raw_input.as_str();
    let mut sum = 0;
    let mut enabled = true;

    while let Ok((remaining, instruction)) = combo_parser(input).finish() {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::DoNot => enabled = false,
            Instruction::Mul(left, right) if enabled => sum += left * right,
            _ => {}
        }

        input = remaining;
    }

    Ok(sum)
}

#[derive(Debug)]
enum Instruction {
    Mul(i64, i64),
    Do,
    DoNot,
}

fn mul_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tuple((tag("mul"), char('('), i64, char(','), i64, char(')')));
    map(parser, |(_, _, left, _, right, _)| {
        Instruction::Mul(left, right)
    })(input)
}

fn do_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tag("do()");
    map(parser, |_| Instruction::Do)(input)
}

fn do_not_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tag("don't()");
    map(parser, |_| Instruction::DoNot)(input)
}

fn combo_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = alt((mul_parser, do_parser, do_not_parser));
    let many_parser = many_till(anychar, parser);
    map(many_parser, |(_, instruction)| instruction)(input)
}
