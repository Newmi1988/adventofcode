use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::time::Instant;
use std::vec;

fn read_input(path: &Path) -> Result<Vec<String>, Error> {
    let file: File = File::open(path)?;
    let f: BufReader<File> = BufReader::new(file);

    let mut input: Vec<String> = Vec::new();

    for line in f.lines() {
        let line = line?;
        input.push(String::from(line));
    }
    Ok(input)
}

fn part1(input: &Vec<String>) -> u32 {
    for (i, line) in input.iter().enumerate() {
        let mut stack: Vec<char> = Vec::new();
        let score_or_char: Option<u32> = line
            .chars()
            .filter_map(|char| match char {
                '(' | '{' | '<' | '[' => {
                    stack.push(char);
                    None
                }
                ')' => (Some('(') != stack.pop()).then(|| 3),
                ']' => (Some('[') != stack.pop()).then(|| 57),
                '}' => (Some('{') != stack.pop()).then(|| 1197),
                '>' => (Some('<') != stack.pop()).then(|| 25137),
                _ => panic!("Unrecognizable character"),
            })
            .next();

        println!("Line: {:?}", score_or_char);
    }

    0_u32
}

fn main() {
    let path: &Path = Path::new("./test");
    let input: Vec<String> = if let Ok(vec) = read_input(path) {
        vec
    } else {
        panic!("Could not read input file")
    };

    println!("{:?}", input);

    let part_one_time = Instant::now();
    let part_one_result: u32 = part1(&input);
    println!(
        "Result Part 1: {} ({}µs)",
        part_one_result,
        part_one_time.elapsed().as_micros()
    );
}
