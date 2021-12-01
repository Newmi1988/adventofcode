use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;

fn read_input(path: &Path) -> Result<Vec<i64>, Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    let mut v = Vec::new();

    for line in f.lines() {
        let line = line?;
        let i = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(i)
    }
    Ok(v)
}

fn get_increase(v: &Vec<i64>) -> i32 {
    let mut res: i32 = 0;
    let mut last_value = v.first().unwrap().clone();
    for i in v.into_iter() {
        if i > &last_value {
            res += 1;
        }
        last_value = *i;
    }

    return res;
}

fn count_sliding_window(v: Vec<i64>) -> i32 {
    let mut cnt: i32 = 0;
    let mut last_window_sum: i64 = v.iter().take(3).sum();
    for w in v.windows(3) {
        let current_window_sum: i64 = w.iter().sum();
        if current_window_sum > last_window_sum {
            cnt += 1
        }
        last_window_sum = current_window_sum;
    }

    return cnt;
}

fn main() {
    let path = Path::new("./input");
    let vec = read_input(path).unwrap();

    let counter = get_increase(&vec);

    println!("Found {} increases", counter);

    let sliding_window_counter = count_sliding_window(vec);
    println!(
        "Found {} increased in sliding windows",
        sliding_window_counter
    );
}
