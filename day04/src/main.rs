use std::fs::{self};

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
    let grid = {
        let width = {
            let Some(row) = raw_data.lines().map(|line| line.trim()).next() else {
                return Ok(0);
            };

            row.len()
        };

        let data = raw_data
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<_>>();

        Grid::from_vec(data, width)
    };

    let iter = (0..grid.rows())
        .flat_map(|row_index| {
            grid.iter_row(row_index)
                .enumerate()
                .map(move |(col_index, character)| ((col_index, row_index), character))
        })
        .filter_map(|(coords, character)| {
            if *character == 'X' {
                Some(coords)
            } else {
                None
            }
        });

    let sum = iter.map(|coord| xmas_word_count(&grid, coord)).sum();

    Ok(sum)
}

fn part2() -> anyhow::Result<usize> {
    let raw_data = fs::read_to_string("./input.txt")?;
    let grid = {
        let width = {
            let Some(row) = raw_data.lines().map(|line| line.trim()).next() else {
                return Ok(0);
            };

            row.len()
        };

        let data = raw_data
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<_>>();

        Grid::from_vec(data, width)
    };

    let iter = (0..grid.rows())
        .flat_map(|row_index| {
            grid.iter_row(row_index)
                .enumerate()
                .map(move |(col_index, character)| ((col_index, row_index), character))
        })
        .filter_map(|(coords, character)| {
            if *character == 'A' {
                Some(coords)
            } else {
                None
            }
        });

    let sum = iter.map(|coord| crossed_mas(&grid, coord)).sum();

    Ok(sum)
}

fn xmas_word_count(grid: &Grid<char>, (x, y): (usize, usize)) -> usize {
    let directions = (-1..2).flat_map(|y| {
        (-1..2).filter_map(move |x| if (x, y) == (0, 0) { None } else { Some((x, y)) })
    });

    let coords = directions.map(|(dir_x, dir_y)| {
        [
            (x as isize + (dir_x * 0), y as isize + (dir_y * 0)),
            (x as isize + (dir_x * 1), y as isize + (dir_y * 1)),
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
            (x as isize + (dir_x * -1), y as isize + (dir_y * -1)),
            (x as isize + (dir_x * 0), y as isize + (dir_y * 0)),
            (x as isize + (dir_x * 1), y as isize + (dir_y * 1)),
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
