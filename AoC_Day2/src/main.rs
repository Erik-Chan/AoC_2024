use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Define the file path directly in the code
    let file_path = "input.txt";
    let mut safety_counter = 0;

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);
        let mut report_array: Vec<i32>;

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    report_array = line.split_whitespace()
                                                  .map(|s| s.parse::<i32>().expect("parse error"))
                                                  .collect();
                    if safety_check(&report_array) == "Safe" {
                        safety_counter += 1;
                    }
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
    }
    print!("{}",safety_counter);
}

fn safety_check(v: &Vec<i32>) -> &str {

    let mut increasing: bool = true;
    let mut decreasing: bool = true;
    let mut prev: i32 = match v.first() {
        Some(&val) => val,
        None => -100
    };
    let mut diff: i32;
    for &curr in &v[1..] {
        diff = curr - prev;
        if diff.abs() > 0 && diff.abs() < 4 {
            if diff > 0 {
                decreasing = false
            }
            else if diff < 0{
                increasing = false
            }
        }
        else {
            return "Unsafe";
        }

        if !decreasing && !increasing {
            return "Unsafe";
        }
        prev = curr
    }
    return "Safe";
}