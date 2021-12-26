use core::panic;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::ops::Sub;
use std::path::Path;
use std::time::Instant;
use std::vec;

fn read_input(path: &Path) -> Result<Vec<Vec<u32>>, Error> {
    let file: File = File::open(path)?;
    let f: BufReader<File> = BufReader::new(file);

    const RADIX: u32 = 10;
    let mut heightmap: Vec<Vec<u32>> = Vec::new();

    for line in f.lines() {
        let line = line?;
        let mut row: Vec<u32> = Vec::new();
        row.push(u32::MAX);
        let t: String = String::from(line);
        t.chars().for_each(|c| row.push(c.to_digit(RADIX).unwrap()));
        row.push(u32::MAX);
        heightmap.push(row);
    }
    // pad first row and last row with max
    let row_padding: Vec<u32> = vec![u32::MAX; heightmap[0].len()];

    heightmap.insert(0, row_padding.clone());
    heightmap.push(row_padding);

    Ok(heightmap)
}

fn part1(heightmap: &Vec<Vec<u32>>) -> u32 {
    let mut found_minumum: Vec<u32> = Vec::new();
    for i in 0..(heightmap.len() - 2) as i32 {
        for j in 0..(heightmap[0].len() - 2) as i32 {
            let mut search_window: HashMap<(usize, usize), u32> = HashMap::new();

            // current location values
            let current_position_value: u32 = heightmap[(i + 1) as usize][(j + 1) as usize].clone();

            for x in 0_i32..=2_i32 {
                let search_x: usize = (i + x).try_into().unwrap();
                let search_y: usize = (j + 1).try_into().unwrap();
                let value_or_passing: u32 = heightmap[search_x][search_y];
                if value_or_passing != u32::MAX {
                    search_window.insert((search_x, search_y), value_or_passing);
                }
            }

            for y in vec![0, 2] {
                let search_x: usize = (i + 1).try_into().unwrap();
                let search_y: usize = (j + y).try_into().unwrap();
                let value_or_passing: u32 = heightmap[search_x][search_y];
                if value_or_passing != u32::MAX {
                    search_window.insert((search_x, search_y), value_or_passing);
                }
            }

            let window_min = search_window.values().min().unwrap();
            if current_position_value == *window_min
                && search_window.values().max().unwrap() != window_min
            {
                println!("{:?}", search_window);
                println!("Minimum found : {}", current_position_value);
                found_minumum.push(current_position_value)
            }
        }
    }

    for v in found_minumum.iter_mut() {
        *v += 1
    }

    found_minumum.iter().sum()
}

fn main() {
    let path: &Path = Path::new("./input");

    let input_heightmap: Vec<Vec<u32>> = if let Ok(vec) = read_input(path) {
        vec
    } else {
        panic!("Could read input file")
    };

    let part_one_time = Instant::now();
    let part_one_result: u32 = part1(&input_heightmap);
    println!(
        "Result Part 1: {} ({}µs)",
        part_one_result,
        part_one_time.elapsed().as_micros()
    );
}
