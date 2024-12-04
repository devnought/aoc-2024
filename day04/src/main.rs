use std::fs::{self};

use anyhow::anyhow;
use grid::Grid;

fn main() -> anyhow::Result<()> {
    let part1 = part1()?;
    let part2 = part2()?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn part1() -> anyhow::Result<usize> {
    let raw_data = fs::read_to_string("./input.txt")?;
    let grid = build_grid(raw_data)?;
    let iter = build_main_iter(&grid, 'X');
    let sum = iter.map(|coord| xmas_word_count(&grid, coord)).sum();

    Ok(sum)
}

fn part2() -> anyhow::Result<usize> {
    let raw_data = fs::read_to_string("./input.txt")?;
    let grid = build_grid(raw_data)?;
    let iter = build_main_iter(&grid, 'A');
    let sum = iter.map(|coord| crossed_mas(&grid, coord)).sum();

    Ok(sum)
}

fn build_grid(input: String) -> anyhow::Result<Grid<char>> {
    let width = {
        let Some(row) = input.lines().map(|line| line.trim()).next() else {
            return Err(anyhow!("Cannot build grid from empty dataset"));
        };

        row.len()
    };

    let data = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>();

    Ok(Grid::from_vec(data, width))
}

fn build_main_iter(
    grid: &Grid<char>,
    character_origin: char,
) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    (0..grid.rows())
        .flat_map(|row_index| {
            grid.iter_row(row_index)
                .enumerate()
                .map(move |(col_index, character)| ((col_index, row_index), character))
        })
        .filter_map(move |(coords, character)| {
            if *character == character_origin {
                Some(coords)
            } else {
                None
            }
        })
}

fn xmas_word_count(grid: &Grid<char>, (x, y): (usize, usize)) -> usize {
    let directions = (-1..2).flat_map(|y| {
        (-1..2).filter_map(move |x| if (x, y) == (0, 0) { None } else { Some((x, y)) })
    });

    let coords = directions.map(|(dir_x, dir_y)| {
        [
            (x as isize, y as isize),
            (x as isize + dir_x, y as isize + dir_y),
            (x as isize + (dir_x * 2), y as isize + (dir_y * 2)),
            (x as isize + (dir_x * 3), y as isize + (dir_y * 3)),
        ]
    });

    let words = coords
        .map(|coords| {
            coords
                .into_iter()
                .filter_map(|(x, y)| grid.get(y, x))
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    words.iter().filter(|w| *w == "XMAS").count()
}

fn crossed_mas(grid: &Grid<char>, (x, y): (usize, usize)) -> usize {
    let directions = (-1..2).flat_map(|y: isize| {
        (-1..2).filter_map(move |x: isize| {
            if x.abs() == 1 && y.abs() == 1 {
                Some((x, y))
            } else {
                None
            }
        })
    });

    let coords = directions.map(|(dir_x, dir_y)| {
        [
            (x as isize + -dir_x, y as isize + -dir_y),
            (x as isize, y as isize),
            (x as isize + dir_x, y as isize + dir_y),
        ]
    });

    let words = coords
        .map(|coords| {
            coords
                .into_iter()
                .filter_map(|(x, y)| grid.get(y, x))
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    words.iter().filter(|w| *w == "MAS").count() / 2
}
