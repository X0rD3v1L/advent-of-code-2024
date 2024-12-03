use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use regex::Regex;


fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("src/input.txt");
    
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for caps in re.captures_iter(&line) {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            
            sum += num1 * num2;
        }
    }
    

    println!("The total sum of all multiplications is: {}", sum);

    Ok(())
}
