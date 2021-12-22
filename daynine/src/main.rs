use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

fn read_input(path: &Path) -> Result<i32,Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);

    let mut heightmap : Vec<Vec<u16>> = Vec::new();

    for line in f.lines() {
        let line = line?;
        let mut row : Vec<u16> = Vec::new();
        let t : String = String::from(line);
        println!("{}",t);
        for c in t.chars() {
            println!("{}",c);
            let z = (c.to_string()).parse::<u16>().unwrap();
            row.push(z);
            println!("{:?}",row);
        }
        heightmap.push(row);
    }

    println!("{:?}",heightmap);
    
    Ok(0_i32)
}

// fn part1(heightmap: &Vec<Vec<u8>>




fn main() {

    let path = Path::new("./input");

    let i = read_input(path);

}
