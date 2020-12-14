use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, Program};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn movement(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn turn_left(&mut self) {
        *self = match *self {
            Direction::Up => Direction:: Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = match *self {
            Direction::Up => Direction:: Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn run_drawing_program(mut program: Program, initial: i64) -> Res<HashMap<(i32, i32), i64>> {
    let mut direction = Direction::Up;
    let mut position = (0, 0);
    let mut panels: HashMap<(i32, i32), i64> = HashMap::new();

    panels.insert(position, initial);

    let mut running = true;
    while running {
        // Input: color of the currect panel, which defaults to black
        let input = panels.get(&position).cloned().unwrap_or(0);

        // Run program until it outputs something
        let mut output = Vec::new();
        while running && output.len() < 2 {
            running = program.step(|| Ok(input), |i| { output.push(i); Ok(()) })?;
        }
        if !running {
            break;
        }

        // Paint current panel
        panels.insert(position, output[0]);

        // Turn
        match output[1] {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            v => return Err(format!("Invalid turn instruction {}", v).into()),
        }

        // Move forward
        position = (
            position.0 + direction.movement().0,
            position.1 + direction.movement().1,
        );
    }

    Ok(panels)
}

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day11.txt")?);

    // Read the program
    let program = Program::from_reader(file)?;

    // Part 1
    {
        let panels = run_drawing_program(program.clone(), 0)?;
        println!("Painted {} panels", panels.len());
    }

    // Part 2
    {
        let panels = run_drawing_program(program.clone(), 1)?;

        // Compute size
        let mut min_x = 9_999;
        let mut max_x = -9_999;
        let mut min_y = 9_999;
        let mut max_y = -9_999;
        for &(x, y) in panels.keys() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        // Draw
        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                // Mirror for some reason
                let x = min_x + max_x - x;

                let color = panels.get(&(x, y)).cloned().unwrap_or(0);
                let chr = match color {
                    0 => '.',
                    1 => '#',
                    _ => '?',
                };
                print!("{}", chr);
            }
            println!("");
        }
    }

    Ok(())
}
