use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Copy, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position{ x : 0, y : 0}
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl Eq for Position {}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Position,
    end: Position,
    index: usize,
}

impl Line {
    fn is_vertical(self) -> bool {
        return self.start.x == self.end.x;
    }

    fn is_horizontal(self) -> bool {
        return self.start.y == self.end.y;
    }

    fn is_diag(self) -> bool {
        return ((self.start.y - self.end.y).abs() / (self.start.x - self.end.x).abs()) == 1
    }
}

impl Iterator for Line {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = Position::new();
        if self.is_horizontal() {
            // println!("{:?}", self);
            if self.index > (self.end.x - self.start.x).abs() as usize {
                return None;
            } else {
                if self.start.x > self.end.x {
                        p.x = self.start.x - self.index as i32;
                        p.y = self.start.y;
                } else {
                        p.x= self.start.x + self.index as i32;
                        p.y = self.start.y;
                }
            }
        } else if self.is_vertical() {
            if self.index > (self.end.y - self.start.y).abs() as usize {
                return None;
            } else {
                if self.start.y > self.end.y {
                        p.x=self.start.x;
                        p.y= self.start.y - self.index as i32;
                } else {
                        p.x=self.start.x;
                        p.y= self.start.y + self.index as i32;
                }
            }
        } else {
            return None;
        }
        self.index += 1;
        Some(p)
    }
}

fn read_input(path: &Path) -> Result<Vec<Line>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let v: Vec<Line> = f
        .lines()
        .filter_map(|l| {
            let l = l.ok()?;
            let (a, b) = l.trim().split_once(" -> ")?;
            let (a_1, a_2) = a.split_once(',')?;
            let (b_1, b_2) = b.split_once(',')?;
            let (start_x, start_y) = (a_1.parse().ok()?, a_2.parse().ok()?);
            let (end_x, end_y) = (b_1.parse().ok()?, b_2.parse().ok()?);
            let start_position = Position {
                x: start_x,
                y: start_y,
            };
            let end_position = Position { x: end_x, y: end_y };
            Some(Line {
                start: start_position,
                end: end_position,
                index: 0,
            })
        })
        .collect();

    Ok(v)
}

fn part1(lines: &mut Vec<Line>) -> i32 {
    lines.retain(|line| line.start.x == line.end.x || line.start.y == line.end.y);

    println!("{}", lines.len());

    let mut points: HashMap<Position, i32> = HashMap::new();
    for line in lines {
        // println!("{:?}", line);

        for pos in line {
            // add counts to position
            let counter = points.entry(pos).or_insert(0);
            *counter += 1
        }

    }
    points.retain(|_,v| *v >= 2);
    points.len() as i32
}

// fn part2(lines: &mut Vec<Line>) -> i32 {
//     lines.retain(|line| *line.is_vertical())

//     0_i32
// }

fn main() {
    let path = Path::new("./input");
    let mut lines = read_input(path).unwrap();
    println!("{}", lines.len());

    // println!("{:?}", lines);

    // part 1
    let overlaps = part1(&mut lines);
    println!("Overlaps : {}", overlaps);
}
