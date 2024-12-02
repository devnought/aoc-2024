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

    let data = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line).finish().ok().map(|(_, num)| num))
        .collect::<Vec<_>>();

    let safe = data
        .iter()
        .map(|report| {
            let data = report
                .as_slice()
                .windows(2)
                .map(|s| s[0] - s[1])
                .collect::<Vec<_>>();

            let all_increasing_decreasing =
                data.iter().map(|num| num.signum()).sum::<i64>().abs() == (report.len() as i64 - 1);
            let acceptable_diff = data.iter().all(|num| (1..=3).contains(&num.abs()));

            if all_increasing_decreasing && acceptable_diff {
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
        .filter_map(|line| parser(&line).finish().ok().map(|(_, num)| num))
        .collect::<Vec<_>>();

    let safe = data
        .into_iter()
        .map(is_report_safe_dampener)
        .map(|value| match value {
            true => 1,
            false => 0,
        })
        .sum();

    Ok(safe)
}

fn is_report_safe_dampener(report: Vec<i64>) -> bool {
    let data = build_diffs(&report);

    // SAFE!
    if are_diffs_safe(&data, report.len()) {
        return true;
    }

    // Ok now build new permutations removing single bad levels
    let filters = 0..report.len();
    for filter in filters.into_iter() {
        let mut new_report = report.clone();
        new_report.remove(filter);

        let new_data = build_diffs(&new_report);

        if are_diffs_safe(&new_data, new_report.len()) {
            return true;
        }
    }

    false
}

fn build_diffs(report: &[i64]) -> Vec<LevelDiff> {
    report
        .windows(2)
        .map(|s| {
            let diff = s[0] - s[1];
            let diff_abs = diff.abs();

            if (1..=3).contains(&diff_abs) {
                LevelDiff::Ok(diff)
            } else {
                LevelDiff::Bad(diff)
            }
        })
        .collect::<Vec<_>>()
}

fn are_diffs_safe(data: &[LevelDiff], report_len: usize) -> bool {
    let all_increasing_decreasing = data
        .iter()
        .map(|num| match num {
            LevelDiff::Ok(diff) => diff.signum(),
            LevelDiff::Bad(diff) => diff.signum(),
        })
        .sum::<i64>()
        .abs();

    let all_increasing_decreasing = all_increasing_decreasing == (report_len as i64 - 1);
    let has_bad_level = data.iter().any(|level| matches!(level, LevelDiff::Bad(_)));

    all_increasing_decreasing && !has_bad_level
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LevelDiff {
    Ok(i64),
    Bad(i64),
}

fn parser(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = tuple((i64, space0));
    let repeating = many1(parser);
    map(repeating, |r| r.into_iter().map(|(num, _)| num).collect())(input)
}
