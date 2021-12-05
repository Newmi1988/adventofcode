use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
struct Position {
    x : i32,
    y : i32
}

#[derive(Debug)]
struct Line {
    start : Position,
    end : Position
}

fn read_input(path: &Path) -> Result<Vec<Line>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let v : Vec<Line> = f
    .lines()
    .filter_map(|l| {
        let l = l.ok()?;
        let (a, b) = l.trim().split_once(" -> ")?;
        let (a_1, a_2) = a.split_once(',')?;
        let (b_1, b_2) = b.split_once(',')?;
        let (start_x, start_y) = (a_1.parse().ok()?, a_2.parse().ok()?);
        let (end_x, end_y) = (b_1.parse().ok()?, b_2.parse().ok()?);
        let start_position = Position{x:start_x,y:start_y};
        let end_position = Position{x:end_x,y:end_y};
        Some(Line{start : start_position, end : end_position})
    })
    .collect();

    Ok(v)
    
}

fn main() {
    let path = Path::new("./input");
    let s = read_input(path).unwrap();

    println!("{:?}", s);
    
}
