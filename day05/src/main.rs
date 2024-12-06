use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{self},
};

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i64},
    combinator::{eof, map, opt},
    multi::many1,
    sequence::{separated_pair, tuple},
    Finish, IResult, Parser,
};

fn main() -> anyhow::Result<()> {
    let part1 = part1()?;
    let part2 = part2()?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

struct Parsed {
    page_order_rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

fn part1() -> anyhow::Result<i64> {
    let input = fs::read_to_string("./input.txt")?;
    let Parsed {
        page_order_rules,
        updates,
    } = parse_input(input)?;

    let sum = updates
        .iter()
        .filter(|update| are_pages_ordered(update, &page_order_rules))
        .map(|update| {
            let middle_index = update.len() / 2;
            update[middle_index]
        })
        .sum();

    Ok(sum)
}

fn part2() -> anyhow::Result<i64> {
    let input = fs::read_to_string("./input.txt")?;
    let Parsed {
        page_order_rules,
        mut updates,
    } = parse_input(input)?;

    let sum = updates
        .iter_mut()
        .filter(|update| !are_pages_ordered(update, &page_order_rules))
        .map(|update| {
            update.sort_by(|&update_left, &update_right| {
                page_order_rules
                    .iter()
                    .filter_map(|&(ord_left, ord_right)| {
                        match (
                            ord_left == update_left && ord_right == update_right,
                            ord_left == update_right && ord_right == update_left,
                        ) {
                            (true, false) => Some(Ordering::Less),
                            (false, true) => Some(Ordering::Greater),
                            _ => None,
                        }
                    })
                    .next()
                    .unwrap_or(Ordering::Equal)
            });

            let middle_index = update.len() / 2;
            update[middle_index]
        })
        .sum();

    Ok(sum)
}

fn are_pages_ordered(update: &[i64], page_order_rules: &[(i64, i64)]) -> bool {
    let order_map = update
        .iter()
        .enumerate()
        .map(|(index, &page)| (page, index))
        .collect::<HashMap<_, _>>();

    page_order_rules
        .iter()
        .filter_map(|(left, right)| Some((*order_map.get(left)?, *order_map.get(right)?)))
        .all(|(left, right)| left < right)
}

fn parse_input(input: String) -> anyhow::Result<Parsed> {
    parser(&input)
        .finish()
        .map(|(_, data)| data)
        .map_err(|e| anyhow!("Could not parse input: {e}"))
}

fn parser(input: &str) -> IResult<&str, Parsed> {
    let parser = tuple((order_pairs, newline, page_numbers_lines));
    map(parser, |(page_order_rules, _, updates)| Parsed {
        page_order_rules,
        updates,
    })(input)
}

fn newline(input: &str) -> IResult<&str, ()> {
    alt((char('\n').map(|_| ()), tag("\r\n").map(|_| ())))(input)
}

fn order_pairs(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let pairs = separated_pair(i64, char('|'), i64);
    let pairs_line = tuple((pairs, opt(newline)));
    many1(pairs_line.map(|(pair, _)| pair))(input)
}

fn page_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let page_number = tuple((i64, opt(char(',')))).map(|(num, _)| num);
    many1(page_number)(input)
}

fn page_numbers_lines(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let parser = tuple((page_numbers, alt((newline, eof.map(|_| ())))));
    many1(parser.map(|(page_numbers, _)| page_numbers))(input)
}
