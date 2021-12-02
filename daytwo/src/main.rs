use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Inputs {
    direction: String,
    steps: i32,
}

fn read_input(path: &Path) -> Result<Vec<Inputs>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut inputs = Vec::new();

    for line in f.lines() {
        let line = line?;
        let l: Vec<&str> = line.trim().split(" ").collect();

        let direction: String = l.first().unwrap().to_string();
        let steps: i32 = l.last().unwrap().to_string().parse::<i32>().unwrap();

        inputs.push(Inputs {
            direction: direction,
            steps: steps,
        });
    }
    Ok(inputs)
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn start() -> Position {
        return Position { x: 0, y: 0 };
    }

    pub fn move_ship(&mut self, i: &Inputs) {

        match i {
            &Inputs { ref direction, steps: _ } if direction == "forward" => self.x += i.steps,
            &Inputs { ref direction, steps: _ } if direction == "up" => self.y -= i.steps,
            &Inputs { ref direction, steps: _ } if direction == "down" => self.y += i.steps,
            &_ => println!("Error")
        }

    }

    fn product(self) -> i32 {
        self.x * self.y 
    }
}

fn move_ship(inputs: &Vec<Inputs>) -> Position {
    let mut pos = Position::start();
    for i in inputs.iter() {
        pos.move_ship(i);
    }

    return pos;
}

fn main() {
    let path = Path::new("./input");
    let inputs = read_input(path).unwrap();

    // println!("{:?}", inputs);

    let p = move_ship(&inputs);
    println!("{:?}", p);
    println!("{:?}", p.product());
}
