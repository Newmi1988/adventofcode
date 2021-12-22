use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

fn read_input(path: &Path) -> Result<Vec<Vec<u32>>,Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
 
    const RADIX: u32 = 10;
    let mut heightmap : Vec<Vec<u32>> = Vec::new();

    for line in f.lines() {
        let line = line?;
        let mut row : Vec<u32> = Vec::new();
        let t : String = String::from(line);
        for c in t.chars() {
            row.push(c
                .to_digit(RADIX)
                .unwrap()
            )
        }
        heightmap.push(row);
    }

    println!("{:?}",heightmap);
    
    Ok(heightmap)
}

// fn part1(heightmap: &Vec<Vec<u32>>) -> u32 {

// }


fn main() {

    let path = Path::new("./input");

    let i: Result<Vec<Vec<u32>>, Error> = read_input(path);

}
