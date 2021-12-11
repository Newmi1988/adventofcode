use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

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

fn make_fish(input: &Vec<i32>, days: i32) -> usize {
    let mut age_counts = [0; 9];

    for age in input {
        age_counts[*age as usize] += 1;
    }

    println!("Initial age counts {:?}", age_counts);

    // instead of extending the array rotate through it
    for _ in 1..=days {
        age_counts.rotate_left(1);
        // add the numbers of fish with delay of 2 to slot 6 (i will totally forget about this logic later...)
        age_counts[6] += age_counts[8];
    }

    age_counts.iter().sum()
}

fn main() {
    let path = Path::new("./input");
    // let path = Path::new("./test");
    let mut input = read_input(path).unwrap();
    println!("{:?}", input);

    // part 1
    let answer_part_one = make_fish(&mut input, 80);
    println!("Number of fish : {}", answer_part_one);

    // part 2
    let answer_part_two = make_fish(&mut input, 256);
    println!("Number of fish : {}", answer_part_two);
}
