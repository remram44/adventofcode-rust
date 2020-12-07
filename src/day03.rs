use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open("inputs/day03.txt")?;

    // Wrap it in a buffered reader, so we can read lines
    let file = BufReader::new(file);

    // Read the map
    let mut matrix = Vec::new();
    let mut width = None;
    for line in file.lines() {
        let line = line?;

        // Remove line delimiter (\n)
        let trimmed = line.trim_end();
        if trimmed.len() == 0 {
            continue;
        }

        // Set or check the width
        match width {
            Some(value) => if value != trimmed.len() {
                return Err("Line length inconsistent".into());
            }
            None => width = Some(trimmed.len()),
        }

        matrix.extend(trimmed.chars().map(|c| c == '#'));
    }
    let width = match width {
        Some(w) => w,
        None => return Err(IoError::new(
            IoErrorKind::UnexpectedEof,
            "No lines in input",
        ).into()),
    };
    assert_eq!(matrix.len() % width, 0);
    let height = matrix.len() / width;

    {
        let count = count_trees(&matrix, width, height, 3, 1);
        println!("Slope (right 3, down 1) crosses {} trees", count);
    }

    {
        let count =
            count_trees(&matrix, width, height, 1, 1) *
            count_trees(&matrix, width, height, 3, 1) *
            count_trees(&matrix, width, height, 5, 1) *
            count_trees(&matrix, width, height, 7, 1) *
            count_trees(&matrix, width, height, 1, 2);
        println!("The multiplied counts give {}", count);
    }

    Ok(())
}

fn count_trees(
    matrix: &[bool],
    width: usize,
    height: usize,
    slope_x: usize,
    slope_y: usize,
) -> usize {
    let mut count = 0;
    let (mut x, mut y) = (0, 0);
    while y < height {
        if matrix[y * width + x] {
            count += 1;
        }
        x = (x + slope_x) % width;
        y += slope_y;
    }
    count
}
