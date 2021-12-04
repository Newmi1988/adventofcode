use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
struct Board {
    width: i32,
    heigth: i32,
    values: Vec<Vec<i32>>,
}

impl Board {
    // implement col and row checking for win
    fn is_won(n: Vec<i32>) -> bool {
        // go throgh rows and check if in n

        // go through columns check if in n

        true
    }
}

#[derive(Debug)]
struct Games {
    inputs: Vec<i32>,
    boards: Vec<Board>,
}

impl Games {
    fn from_input_file(path: &Path) -> Result<Games, Error> {
        let file = File::open(path)?;
        let f = BufReader::new(file);
        let mut inputs: Vec<i32> = Vec::new();
        let mut board_lines: Vec<Vec<i32>> = Vec::new();

        for line in f.lines() {
            let line = line?;
            if line.contains(",") {
                inputs = line.trim().split(",").map(|s| s.parse().unwrap()).collect();
            } else if line.is_empty() {
                continue;
            } else {
                let row: Vec<i32> = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                board_lines.push(row);
            }
        }

        // generate the boards
        let dimensions: usize = board_lines.first().unwrap().len();
        println!("{}", dimensions);

        let mut boards: Vec<Board> = Vec::new();
        for b in board_lines.windows(dimensions) {
            boards.push(Board {
                width: dimensions as i32,
                heigth: dimensions as i32,
                values: b.to_vec(),
            })
        }

        let g = Games {
            inputs: inputs,
            boards: boards,
        };

        Ok(g)
    }
}

fn main() {
    let path = Path::new("./input");
    let g = Games::from_input_file(path).unwrap();

    println!("{:?}", g)
}
