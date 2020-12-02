use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Essentially str::split_once(), which unfortunately is not stable.
///
/// https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_once
fn split<'a>(
    string: &'a str,
    delimiter: char,
) -> Option<(&'a str, &'a str)> {
    let mut iter = string.splitn(2, delimiter);
    if let Some(left) = iter.next() {
        if let Some(right) = iter.next() {
            return Some((left, right));
        }
    }
    None
}

#[test]
fn test_split_once() {
    assert_eq!(split("remi", 'k'), None);
    assert_eq!(split("remie", 'm'), Some(("re", "ie")));
    assert_eq!(split("remi", 'i'), Some(("rem", "")));
    assert_eq!(split("rémi", 'é'), Some(("r", "mi")));
}

fn parse_line(line: &str) -> Result<(u32, u32, char, &str), Box<dyn Error>> {
    // Read first number
    let (min, line) = split(&line, '-')
        .ok_or("Missing dash")?;
    let min: u32 = min.parse()?;

    // Read second number
    let (max, line) = split(&line, ' ')
        .ok_or("Missing space")?;
    let max: u32 = max.parse()?;

    let mut iter = line.char_indices();

    // Read character
    let (_, chr) = iter.next()
        .ok_or("Missing character")?;

    // Read colon
    match iter.next() {
        Some((_, ':')) => {}
        _ => return Err("Missing colon".into()),
    }

    // Read space
    match iter.next() {
        Some((_, ' ')) => {}
        _ => return Err("Missing space".into()),
    }

    // Read password
    let password = match iter.next() {
        Some((i, _)) => &line[i..],
        _ => return Err("Missing password".into()),
    }.trim_end();

    Ok((min, max, chr, password))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file
    let file = File::open("inputs/day02.txt")?;

    // Wrap it in a buffered reader, so we can read lines
    let file = BufReader::new(file);

    // First part
    let mut valid_passwords = 0;
    for line in file.lines() {
        let line = line?;

        let (min, max, chr, password) = parse_line(&line)?;

        let mut count = 0;
        for c in password.chars() {
            if c == chr {
                count += 1;
            }
        }

        if (min <= count) && (count <= max) {
            valid_passwords += 1;
        }
    }
    println!("Valid passwords for part 1: {}", valid_passwords);

    // Open the file
    let file = File::open("inputs/day02.txt")?;

    // Wrap it in a buffered reader, so we can read lines
    let file = BufReader::new(file);

    // Second part
    let mut valid_passwords = 0;
    for line in file.lines() {
        let line = line?;

        let (pos1, pos2, chr, password) = parse_line(&line)?;

        let mut chars = password.chars();
        let char1 = chars.nth(pos1 as usize - 1)
            .ok_or("Index out of range")?;
        let char2 = chars.nth((pos2 - pos1 - 1) as usize)
            .ok_or("Index out of range")?;

        if char1 == chr {
            if char2 != chr {
                valid_passwords += 1;
            }
        } else if char2 == chr {
            valid_passwords += 1;
        }
    }
    println!("Valid passwords for part 2: {}", valid_passwords);

    Ok(())
}
