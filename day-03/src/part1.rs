use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Number(u32),
    None,
}

fn read_to_2d_array(name: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let char_2d_array: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    io::Result::Ok(char_2d_array)
}

pub fn process(input: &str) -> i32 {
    let char_2d_array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in &char_2d_array {
        for &c in row {
            print!("{} ", c);
        }
        println!();
    }
    
    let map = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, character)| {
            (
                (y as i32, x as i32),
                match character {
                    '.' => Value::None,
                    c if c.is_ascii_digit() => {
                        Value::Number(
                            c.to_digit(10).expect("Should be a number")
                        )
                    },
                    c => Value::Symbol(c),
                }
            )
        })
    })
    .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];

    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last()
                                    .expect("should exist");

                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num
                                )]);
                            }
                        },
                        None => unreachable!("shouldn't happen")
                    }
                },
                None => numbers.push(vec![((*x, *y), *num)])
            }
        }
    }

    // map: entire grid
    // numbers: sequential numbers
    let mut total = 0;
    for num_list in numbers {
        // (x, y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let num_positions = num_list
            .iter()
            .map(|((y, x), _)| (*x, *y))
            .collect::<Vec<(i32, i32)>>();

        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    (outer_pos.0 + pos.1, outer_pos.1 + pos.0)
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = pos_to_check.iter().any(|pos| {
            let value = map.get(pos);
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(4361, process(input));
        Ok(())
    }
}