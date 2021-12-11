use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

fn read_input(path: &Path) -> Result<Vec<i32>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut inputs: Vec<i32> = Vec::new();

    for line in f.lines() {
        let line = line?;
        inputs = line.trim().split(",").map(|s| s.parse().unwrap()).collect();
    }
    Ok(inputs)
}

fn part1(input: &mut Vec<i32>) -> i32 {
    let mut position_fuel_costs: HashMap<i32, i32> = HashMap::new();

    for pos in 0..input.len() {
        let mut other_entries = input.clone();
        other_entries.remove(pos);

        let diff: Vec<i32> = (0..other_entries.len())
            .map(|i| (other_entries[i] - input[pos]).abs())
            .collect();
        let costs: i32 = diff.iter().sum();

        let current_pos = position_fuel_costs.entry(input[pos]).or_insert(0);
        *current_pos = costs;
    }
    let min_value = position_fuel_costs.values().min().unwrap();

    // find key
    let key_min_value = position_fuel_costs
        .iter()
        .find_map(|(key, value)| if value == min_value { Some(key) } else { None })
        .unwrap();

    position_fuel_costs[key_min_value]
}

fn part2(input: &mut Vec<i32>) -> i32 {
    let mut position_fuel_costs: HashMap<i32, i32> = HashMap::new();

    let min_position = *input.iter().min().unwrap();
    let max_position = *input.iter().max().unwrap();

    // instead of using the positions of the craps use all possible in the range
    for pos in min_position..max_position {
        let diff: Vec<i32> = (0..input.len())
            .map(|i| {
                let diff = (input[i] - pos).abs();
                let single_cost = (diff * (diff + 1)) / 2;
                return single_cost;
            })
            .collect();
        let costs: i32 = diff.iter().sum();

        let current_pos = position_fuel_costs.entry(pos).or_insert(0);
        *current_pos = costs;
    }
    let min_value = position_fuel_costs.values().min().unwrap();

    // find key
    let key_min_value = position_fuel_costs
        .iter()
        .find_map(|(key, value)| if value == min_value { Some(key) } else { None })
        .unwrap();

    position_fuel_costs[key_min_value]
}

fn main() {
    // read input
    // let path = Path::new("./test");
    let path = Path::new("./input");
    let mut input = read_input(path).unwrap();

    // part 1
    let part_one_time = Instant::now();
    let answer_part_one = part1(&mut input);
    println!(
        "Answer part two : {} ({}µs)",
        answer_part_one,
        part_one_time.elapsed().as_micros()
    );

    // part 2
    let part_two_time = Instant::now();
    let answer_part_two = part2(&mut input);
    println!(
        "Answer part two : {} ({}µs)",
        answer_part_two,
        part_two_time.elapsed().as_micros()
    );
}
