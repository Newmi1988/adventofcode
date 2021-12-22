use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;
use std::vec;

fn read_input(path: &Path) -> Result<Vec<Vec<u32>>,Error> {
    let file: File = File::open(path)?;
    let f: BufReader<File> = BufReader::new(file);
 
    const RADIX: u32 = 10;
    let mut heightmap : Vec<Vec<u32>> = Vec::new();

    for line in f.lines() {
        let line = line?;
        let mut row : Vec<u32> = Vec::new();
        let t : String = String::from(line);
        t.chars().for_each(|c| {
            row.push(c
                .to_digit(RADIX)
                .unwrap()
            )
        });
        heightmap.push(row);
    }

    println!("{:?}",heightmap);
    
    Ok(heightmap)
}

// fn part1(heightmap: &Vec<Vec<u32>>) -> u32 {

// }


fn main() {

    let path: &Path = Path::new("./input");

    let i: Vec<Vec<u32>> = if let Ok(vec) = read_input(path) {
        vec
    } else {
        panic!("Could read input file")
    };

}
