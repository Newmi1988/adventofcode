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

fn get_binary_position_counts(v: &Vec<String>) -> Vec<HashMap<char,i32>> {
    let mut vhm = Vec::new();
    let string_length = v.first().unwrap().len();
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

    return vhm
}

fn binary_diagnostic(v: &Vec<String>) -> i32 {
    let vhm = get_binary_position_counts(v);
    // println!("{:?}", vhm);

    // set_max =
    let mut max: Vec<char> = Vec::new();
    let mut min: Vec<char> = Vec::new();

    for h in vhm {
        // println!("{:?}", h);

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


    let gamma: i32 = i32::from_str_radix(&max_str, 2).unwrap();
    let epsilon: i32 = i32::from_str_radix(&min_str, 2).unwrap();

    println!("{}", gamma);
    println!("{}", epsilon);

    return gamma * epsilon;
}

fn co_two_scrubber_rating(v : &mut Vec<String>) -> i32 {
    let vhm = get_binary_position_counts(v);

    // println!("{:?}", vhm);

    let oxygen = v;
    let mut co2scrub = oxygen.clone();
    // use the counts to sort the values into 
    for (i,h) in vhm.iter().enumerate() {

        // println!("{} - {:?}",i,h);

        let mut max : char = '0';
        let mut min : char = '1';
        if h[&'0'] < h[&'1'] {
            max = '1';
            min = '0';
        }
        
        if oxygen.len() > 1 {
            oxygen.retain(|x| x.chars().nth(i).unwrap() == max);
        }
        if co2scrub.len() > 1 {
            co2scrub.retain(|x| x.chars().nth(i).unwrap() == min);
        }
        // println!("{:?}",oxygen);
    }

    // println!("{}", oxygen[0]);
    // println!("{}", co2scrub[0]);
    println!("{}",oxygen[0]);
    println!("{}",co2scrub[0]);

    let oxygen_value : i32 = i32::from_str_radix(&oxygen[0],2).unwrap();
    let co2scrub_value : i32 = i32::from_str_radix(&co2scrub[0],2).unwrap();

    println!("{}", oxygen_value);
    println!("{}", co2scrub_value);
    
    return oxygen_value*co2scrub_value
}

fn main() {
    let path = Path::new("./input");
    let mut inputs = read_input(path).unwrap();

    // part 1
    let i = binary_diagnostic(&inputs);
    println!("Part 1: {}", i);

    // part 2
    let j = co_two_scrubber_rating(&mut inputs);
    println!("Part 2: {}",j);
}
