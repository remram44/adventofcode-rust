use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_for_mass(mass: u32) -> u32 {
    let fuel = (mass / 3) as i32 - 2;
    // Don't use negative fuel
    fuel.max(0) as u32
}

#[test]
fn test_fuel_for_mass() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);
}

fn total_fuel_for_mass(mut mass: u32) -> u32 {
    let mut total_fuel = 0;
    loop {
        let fuel = fuel_for_mass(mass);
        if fuel > 0 {
            total_fuel += fuel;
            mass = fuel;
        } else {
            break;
        }
    }
    total_fuel
}

#[test]
fn test_total_fuel_for_mass() {
    assert_eq!(total_fuel_for_mass(14), 2);
    assert_eq!(total_fuel_for_mass(1969), 966);
    assert_eq!(total_fuel_for_mass(100756), 50346);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        // Open the file
        let file = File::open("inputs/day01.txt")?;

        // Wrap it in a buffered reader, so we can read lines
        let file = BufReader::new(file);

        let mut total_fuel = 0;
        for line in file.lines() {
            let line = line?;
            let mass: u32 = line.parse()?;

            total_fuel += fuel_for_mass(mass);
        }
        println!("Fuel: {}", total_fuel);
    }

    {
        // Open the file
        let file = File::open("inputs/day01.txt")?;

        // Wrap it in a buffered reader, so we can read lines
        let file = BufReader::new(file);

        let mut total_fuel = 0;
        for line in file.lines() {
            let line = line?;
            let mass: u32 = line.parse()?;

            total_fuel += total_fuel_for_mass(mass);
        }
        println!("Total fuel: {}", total_fuel);
    }

    Ok(())
}
