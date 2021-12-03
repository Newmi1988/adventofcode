use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

fn read_input(path: &Path) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut v = Vec::new();

    for line in f.lines() {
        let line = line?;
        let l: String = line.trim().to_string();
        v.push(l);
    }
    Ok(v)
}

fn binary_diagnostic(v: &Vec<String>) -> i32 {
    let string_length = v.first().unwrap().len();
    let mut vhm = Vec::new();
    for _ in 0..string_length {
        let mut hm = HashMap::new();
        hm.insert('0', 0);
        hm.insert('1', 0);
        vhm.push(hm)
    }
    for b in v.iter() {
        for (i, v) in b.chars().enumerate() {
            let counter = vhm[i].entry(v).or_insert(0);
            *counter += 1;
        }
    }

    // set_max =
    let mut max: Vec<char> = Vec::new();
    let mut min: Vec<char> = Vec::new();

    for h in vhm {
        println!("{:?}", h);

        if h[&'0'] > h[&'1'] {
            max.push('0');
            min.push('1')
        } else {
            max.push('1');
            min.push('0')
        }
    }

    let max_str: String = max.into_iter().collect();
    let min_str: String = min.into_iter().collect();

    println!("{}", max_str);
    println!("{}", min_str);

    let gamma: i32 = isize::from_str_radix(&max_str, 2).unwrap() as i32;
    let epsilon: i32 = isize::from_str_radix(&min_str, 2).unwrap() as i32;
    return gamma * epsilon;
}

fn main() {
    let path = Path::new("./input");
    let inputs = read_input(path).unwrap();

    // part 1
    let i = binary_diagnostic(&inputs);
    println!("{}", i);
}
