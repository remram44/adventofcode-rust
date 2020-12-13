use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, read_program, run_program};

struct Permutations {
    size: usize,
    elements: usize,
    array: Vec<usize>,
}

impl Permutations {
    fn bump(&mut self, pos: usize) -> bool {
        let mut inc = true;
        loop {
            while self.array[pos] + 1 < self.elements {
                // Increment the element, unless we just looped
                if inc {
                    self.array[pos] += 1;
                } else {
                    inc = true;
                }

                // Check that this element is unique in the array
                let mut unique = true;
                for i in 0..pos {
                    if self.array[i] == self.array[pos] {
                        unique = false;
                        break;
                    }
                }

                // If unique, yield this, otherwise keep incrementing
                if unique {
                    return true;
                }
            }

            // Tried all possible values for this element, reset to 0 and
            // change previous elements recursively
            if pos > 0 && self.bump(pos - 1) {
                self.array[pos] = 0;
                inc = false;
                continue;
            } else {
                // No previous element
                return false;
            }
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.bump(self.size - 1) {
            Some(self.array.clone())
        } else {
            None
        }
    }
}

fn permutations(size: usize, elements: usize) -> Permutations {
    let mut array: Vec<usize> = (0..(size - 1)).collect();
    array.push(size - 2);
    Permutations {
        size,
        elements,
        array,
    }
}

#[test]
fn test_permutations() {
    match permutations(4, 5) {
        Permutations {
            size: 4,
            elements: 5,
            array,
        } => assert_eq!(array, vec![0, 1, 2, 2]),
        _ => panic!(),
    }
    assert_eq!(
        permutations(2, 3).collect::<Vec<_>>(),
        vec![vec![0, 1], vec![0, 2], vec![1, 0], vec![1, 2], vec![2, 0], vec![2, 1]],
    );
    assert_eq!(
        permutations(3, 2).collect::<Vec<_>>(),
        Vec::<Vec<usize>>::new(),
    );
    assert_eq!(
        permutations(3, 3).collect::<Vec<_>>(),
        vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 0, 2], vec![1, 2, 0], vec![2, 0, 1], vec![2, 1, 0]],
    );
}

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day07.txt")?);

    // Read the program
    let program = read_program(file)?;

    let mut best_phases = None;

    // Go through all possible phase settings
    for phases in permutations(5, 5) {
        // Run the programs with those phase settings
        let mut output = 0;
        for phase in &phases {
            let mut program = program.clone();
            // Elements are read in reverse: phase then current output
            let mut inputs = vec![output, *phase as i32];
            run_program(
                &mut program,
                || inputs.pop().ok_or("Read too many inputs".into()),
                |i| { output = i; Ok(()) },
            )?;
        }

        // Update best phases
        match best_phases {
            None => {
                best_phases = Some((output, phases.clone()));
            }
            Some((best_output, _)) => {
                if output > best_output {
                    best_phases = Some((output, phases.clone()));
                }
            }
        }
    }

    let (best_output, best_phases) = best_phases.unwrap();
    println!("Best output: {} for phases {:?}", best_output, best_phases);

    Ok(())
}
