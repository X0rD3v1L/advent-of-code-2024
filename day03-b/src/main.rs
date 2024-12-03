use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("src/input.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mul_re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;

    for line in reader.lines() {
        let line = line?;

        for capture in mul_re.captures_iter(&line) {
            let full_match = capture.get(0).unwrap().as_str();

            if full_match == "do()" {
                enabled = true;
            } else if full_match == "don't()" {
                enabled = false;
            } else if full_match.starts_with("mul(") && enabled {
                let num1: i32 = capture[2].parse().unwrap();
                let num2: i32 = capture[3].parse().unwrap();
                sum += num1 * num2;
            }
        }
    }

    println!("The total sum of all multiplications is: {}", sum);

    Ok(())
}
