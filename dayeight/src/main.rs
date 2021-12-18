use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;


fn read_input(path : &Path) -> Result<(Vec<Vec<String>>,Vec<Vec<String>>),Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for line in f.lines() {
        let line = line?;
        let read_text : Vec<String> = line.trim()
            .split("|")
            .map(str::to_string)
            .collect();

        let i : Vec<String> = read_text.first()
            .unwrap()
            .trim()
            .split(" ")
            .map(str::to_string)
            .collect();

        inputs.push(i);

        let o : Vec<String> = read_text.last()
            .unwrap()
            .trim()
            .split(" ")
            .map(str::to_string)
            .collect();

        outputs.push(o);
    }

    Ok((inputs, outputs))
}

fn part1(signal : Vec<Vec<String>>) -> u32 {

    let mut count : u32 = 0;

    for output in signal {
        for text in output {
            match text.len() {
                7 => count+=1 ,
                4=> count+=1,
                3 => count+=1,
                2=> count+=1,
                _ => continue
            }
        }
    }

    // let a : Vec<u32> = vec![1,2,3];
    // a
    count
}

fn main() {
    let path = Path::new("./test");


    let (i,o) = match read_input(path) {
        Ok(tuple) => (tuple.0, tuple.1),
        _ => panic!("Error reading file")
    };

    println!("{:?}", i);

    let result_part_one = part1(o);
    println!("{}",result_part_one);
}
