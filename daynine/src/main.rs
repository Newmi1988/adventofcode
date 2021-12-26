use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
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
                found_minumum.push(current_position_value)
            }
        }
    }

    for v in found_minumum.iter_mut() {
        *v += 1
    }

    found_minumum.iter().sum()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Area {
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
    area: u32,
}

fn search_area(
    heightmap: &Vec<Vec<u32>>,
    visited: &mut HashMap<(usize, usize), bool>,
    x: usize,
    y: usize,
) {
    if heightmap[x][y] < 9 && visited.contains_key(&(x, y)) == false {
        visited.insert((x, y), true);
        // search the 4 neighbors
        search_area(heightmap, visited, x + 1, y);
        search_area(heightmap, visited, x - 1, y);
        search_area(heightmap, visited, x, y + 1);
        search_area(heightmap, visited, x, y - 1);
    }
}

fn fill_arrea(heightmap: &Vec<Vec<u32>>, x: usize, y: usize) -> HashMap<(usize, usize), bool> {
    let mut possible_area_positions: HashMap<(usize, usize), bool> = HashMap::new();

    search_area(heightmap, &mut possible_area_positions, x, y);

    possible_area_positions
}

fn get_area(heightmap: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<Area> {
    if heightmap[x][y] > 8 {
        return None;
    } else {
        let visited_positions: HashMap<(usize, usize), bool> = fill_arrea(heightmap, x, y);
        let mut x_values: Vec<usize> = Vec::new();
        let mut y_values: Vec<usize> = Vec::new();
        for tuple in visited_positions.keys() {
            x_values.push(tuple.0);
            y_values.push(tuple.1);
        }

        if visited_positions.len() <= 1 {
            None
        } else {
            Some(Area {
                start_x: x_values.iter().min().unwrap_or(&0).clone(),
                end_x: x_values.iter().max().unwrap_or(&0).clone(),
                start_y: y_values.iter().min().unwrap_or(&0).clone(),
                end_y: y_values.iter().min().unwrap_or(&0).clone(),
                area: visited_positions.len() as u32,
            })
        }
    }
}

fn part2(heightmap: &Vec<Vec<u32>>) -> u32 {
    // hashmap to keep track of sizes
    let mut areas: HashMap<Area, u32> = HashMap::new();

    for i in 1..(heightmap.len() - 1) {
        for j in 1..(heightmap[0].len() - 1) {
            // find basins
            if let Some(x) = get_area(heightmap, i, j) {
                areas.insert(x, x.area);
            }
        }
    }

    // sort the areas and then get the three largest
    let mut areas_values: Vec<u32> = Vec::new();
    for v in areas.values() {
        areas_values.push(*v)
    }

    areas_values.sort();
    areas_values.reverse();

    areas_values[0..=2].iter().product()
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

    let part_two_time: Instant = Instant::now();
    let part_two_result: u32 = part2(&input_heightmap);
    println!(
        "Result Part 2: {} ({}µs)",
        part_two_result,
        part_two_time.elapsed().as_micros()
    );
}
