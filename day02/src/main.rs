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
use util::IterWindowIterator;

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

    let data = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line).finish().ok().map(|(_, num)| num));

    let safe = data
        .map(|report| {
            let iter = report.as_slice().windows(2).map(|s| s[0] - s[1]);
            let mut sum = 0;

            for diff in iter {
                if !(1..4).contains(&diff.abs()) {
                    return 0;
                }

                sum += diff.signum();
            }

            if sum.abs() == (report.len() - 1) as i64 {
                1
            } else {
                0
            }
        })
        .sum();

    Ok(safe)
}

fn part2() -> anyhow::Result<i64> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line).finish().ok().map(|(_, num)| num));

    let safe = data
        .map(|report| {
            for index in 0..report.len() {
                let iter = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, num)| if i == index { None } else { Some(*num) })
                    .iter_window::<2>()
                    .map(|[left, right]| left - right);

                let mut sum = 0;

                for diff in iter {
                    if !(1..4).contains(&diff.abs()) {
                        continue;
                    }

                    sum += diff.signum();
                }

                if sum.abs() == (report.len() - 2) as i64 {
                    return 1;
                }
            }

            0
        })
        .sum();

    Ok(safe)
}

fn parser(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = tuple((i64, space0));
    let repeating = many1(parser);
    map(repeating, |r| r.into_iter().map(|(num, _)| num).collect())(input)
}
