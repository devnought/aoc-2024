use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i64},
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
    let raw_input = fs::read_to_string("./input.txt")?;
    let mut input = raw_input.as_str();
    let mut sum = 0;

    loop {
        if input.is_empty() {
            break;
        }

        let Ok((remaining, (left, right))) = mul_parser_part_1(input).finish() else {
            input = &input[1..];
            continue;
        };

        sum += left * right;
        input = remaining;
    }

    Ok(sum)
}

fn part2() -> anyhow::Result<i64> {
    let raw_input = fs::read_to_string("./input.txt")?;
    let mut input = raw_input.as_str();
    let mut sum = 0;
    let mut enabled = true;

    loop {
        if input.is_empty() {
            break;
        }

        let Ok((remaining, instruction)) = combo_parser(input).finish() else {
            input = &input[1..];
            continue;
        };

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

enum Instruction {
    Mul(i64, i64),
    Do,
    DoNot,
}

fn mul_parser_part_1(input: &str) -> IResult<&str, (i64, i64)> {
    let parser = tuple((tag("mul"), char('('), i64, char(','), i64, char(')')));
    map(parser, |(_, _, left, _, right, _)| (left, right))(input)
}

fn mul_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tuple((tag("mul"), char('('), i64, char(','), i64, char(')')));
    map(parser, |(_, _, left, _, right, _)| {
        Instruction::Mul(left, right)
    })(input)
}

fn do_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tuple((tag("do"), char('('), char(')')));
    map(parser, |_| Instruction::Do)(input)
}

fn do_not_parser(input: &str) -> IResult<&str, Instruction> {
    let parser = tuple((tag("don't"), char('('), char(')')));
    map(parser, |_| Instruction::DoNot)(input)
}

fn combo_parser(input: &str) -> IResult<&str, Instruction> {
    alt((mul_parser, do_parser, do_not_parser))(input)
}
