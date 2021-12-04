use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
struct Board {
    width: usize,
    heigth: usize,
    values: Vec<Vec<i32>>,
    wining_number : i32,
    solved : bool
}

impl Board {
    // implement col and row checking for win
    fn is_solved(&mut self,n: Vec<i32>) -> bool {
        if self.solved {
            return self.solved;
        }

        
        // go throgh rows and check if in n
        for row in &self.values {
            if row.iter().all(|item| n.contains(item)) {
                println!("{:?}", self.values);
                self.solved = true;
                self.wining_number = *n.last().unwrap();
                println!("Game in row {:?} solved with inputs {:?}", row, n);
            }
        }

        // go through columns check if in n
        for i in 0..self.heigth {
            let mut column : Vec<i32> = Vec::new();
            for j in 0..self.width {
                column.push(self.values[j][i])
            }

            if column.iter().all(|item| n.contains(item)) {
                println!("{:?}", self.values);
                self.solved = true;
                self.wining_number = *n.last().unwrap();
                println!("Game in column {:?} solved with inputs {:?}", column, n);
            }
        }

        return self.solved
    }

    fn get_answer(&self, n : Vec<i32>) -> i32 {
        // get all values not in solution
        let mut f : Vec<i32> = self.values
            .iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();


        f.retain(|&x| !n.contains(&x) );

        let f_sum : i32 = f.iter().sum();
        // println!("Sum : {}, Winning_number : {}", f_sum, self.wining_number);
        let res : i32 = self.wining_number * f_sum;

        return res
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

        let mut boards: Vec<Board> = Vec::new();
        for b in board_lines.chunks(dimensions) {
            let mut board : Board = Board {
            width: dimensions,
            heigth: dimensions,
            values: b.to_vec(),
            solved : false,
            wining_number: 0
        };
            boards.push(board)
        }

        let g = Games {
            inputs: inputs,
            boards: boards,
        };

        Ok(g)
    }

    fn get_first_solved(&mut self) -> i32 {
        let mut res : i32 = 0;
        for i in 5..self.inputs.len() {
            for board in &mut self.boards {
                if board.is_solved(self.inputs[0..i].to_vec()) {
                    let s : i32 = board.get_answer(self.inputs[0..i].to_vec());
                    return s
                }
            }
        }
        
        return res
    }
    
    fn get_last_solved(&mut self) -> i32 {
        let mut res : i32 = 0;
        for i in 5..self.inputs.len() {
            for board in &mut self.boards {
                if board.is_solved(self.inputs[0..i].to_vec()) {
                    res = board.get_answer(self.inputs[0..i].to_vec());
                }
            }
            // drop all already solved boards
            self.boards.retain(|x| x.solved == false)
        }

        return res
    }
}

fn main() {
    let path = Path::new("./input");
    let mut g = Games::from_input_file(path).unwrap();

    // part 1
    let answer_part_1 = g.get_first_solved();
    println!("{}",answer_part_1);

    // part 2
    let answer_part_2 = g.get_last_solved();
    println!("{}",answer_part_2)
}
