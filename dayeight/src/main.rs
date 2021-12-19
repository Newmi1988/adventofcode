use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

fn read_input(path: &Path) -> Result<(Vec<Vec<String>>, Vec<Vec<String>>), Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for line in f.lines() {
        let line = line?;
        let read_text: Vec<String> = line.trim().split("|").map(str::to_string).collect();

        let i: Vec<String> = read_text
            .first()
            .unwrap()
            .trim()
            .split(" ")
            .map(str::to_string)
            .collect();

        inputs.push(i);

        let o: Vec<String> = read_text
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(str::to_string)
            .collect();

        outputs.push(o);
    }

    Ok((inputs, outputs))
}

fn part1(signal: &Vec<Vec<String>>) -> u32 {
    let mut count: u32 = 0;

    for output in signal {
        for text in output {
            match text.len() {
                7 => count += 1,
                4 => count += 1,
                3 => count += 1,
                2 => count += 1,
                _ => continue,
            }
        }
    }

    count
}

fn find_key_for_value<'a>(map: &'a HashMap<String, u8>, value: &u8) -> Option<&'a String> {
    map.iter()
        .find_map(|(key, &val)| if val == *value { Some(key) } else { None })
}

fn contains_numbers(target: &String, query: &String) -> bool {
    let mut hits: i32 = 0;
    for c in query.chars() {
        if target.contains(c) {
            hits += 1
        }
    }
    let res = hits == query.len() as i32;
    res
}

fn intersection(a: &String, b: &String) -> String {
    if a.len() > b.len() {
        return b.chars().filter(|&x| a.contains(x)).collect();
    } else {
        return a.chars().filter(|&x| b.contains(x)).collect();
    }
}

fn part2(signal: (Vec<Vec<String>>, Vec<Vec<String>>)) -> u32 {
    let mut count: u32 = 0;

    for (input, output) in signal.0.iter().zip(signal.1) {
        let mut mapping: HashMap<String, u8> = HashMap::new();
        for text in input {
            let mut sorted_chars: Vec<char> = text.chars().collect();
            sorted_chars.sort_by(|b, a| b.cmp(a));
            let key = sorted_chars.into_iter().collect::<String>();

            match text.len() {
                7 => mapping.insert(key, 8),
                4 => mapping.insert(key, 4),
                3 => mapping.insert(key, 7),
                2 => mapping.insert(key, 1),
                _ => continue,
            };
        }

        let encoding_seven = find_key_for_value(&mapping, &7).unwrap().clone();
        let encoding_one = find_key_for_value(&mapping, &1).unwrap().clone();
        let encoding_four = find_key_for_value(&mapping, &4).unwrap().clone();
        for text in input {
            let mut sorted_chars: Vec<char> = text.chars().collect();
            sorted_chars.sort_by(|b, a| b.cmp(a));
            let key = sorted_chars.into_iter().collect::<String>();

            match text.len() {
                5 => {
                    // handle all cases with length 5
                    // can be 5,3,2
                    if contains_numbers(&text, &encoding_one) {
                        mapping.insert(key, 3)
                    } else if intersection(&text.clone(), &encoding_four.clone()).len() > 2 {
                        mapping.insert(key, 5)
                    } else {
                        mapping.insert(key, 2)
                    }
                }
                6 => {
                    // 0,9,6
                    if contains_numbers(&text, &encoding_seven) {
                        // text : 0 or 9
                        if intersection(&text, &encoding_four).len() < 4 {
                            // if 0 in 8 -> 0
                            mapping.insert(key, 0)
                        } else {
                            // text -> 9
                            mapping.insert(key, 9)
                        }
                    } else {
                        mapping.insert(key, 6)
                    }
                }
                _ => continue,
            };
        }

        let mut n: String = String::from("");
        for o in output {
            // sort
            //
            let mut sorted_chars: Vec<char> = o.chars().collect();
            sorted_chars.sort_by(|b, a| b.cmp(a));
            let key = sorted_chars.into_iter().collect::<String>();

            n += &*mapping.get(&key).unwrap().to_string();
        }

        count += n.parse::<u32>().unwrap();
    }

    count
}
fn main() {
    // let path = Path::new("./test");
    let path = Path::new("input");

    let (i, o) = match read_input(path) {
        Ok(tuple) => (tuple.0, tuple.1),
        _ => panic!("Error reading file"),
    };

    let part_one_time = Instant::now();
    let result_part_one = part1(&o);
    println!(
        "Answer part one : {} ({}µs)",
        result_part_one,
        part_one_time.elapsed().as_micros()
    );

    let part_two_time = Instant::now();
    let result_part_two = part2((i, o));
    println!(
        "Answer part two : {} ({}µs)",
        result_part_two,
        part_two_time.elapsed().as_micros()
    );
}
