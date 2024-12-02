use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

fn is_monotonic(nums: &Vec<i32>) -> i32 {
    if is_increasing(&nums) || is_decreasing(&nums) {
        return 1;
    }
    0
}

fn is_increasing(nums: &Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }
    
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_decreasing(nums: &Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }
    
    for i in 0..nums.len() - 1 {
        let diff = nums[i] - nums[i + 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("src/input.txt");
    
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut safe_score = 0;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        safe_score += is_monotonic(&nums);
    }

    println!("Safe score: {}", safe_score);

    Ok(())
}