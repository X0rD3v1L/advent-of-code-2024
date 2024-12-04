use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("src/input.txt");
    
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
        map.push(row);
    }

    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let directions = [
                [(0, 0), (1, -1), (2, -2), (3, -3)],
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                [(0, 0), (0, 1), (0, 2), (0, 3)],
            ];

            for dir in directions.iter() {
                let mut valid = true;
                let mut word = [0; 4];

                for (i, &(dx, dy)) in dir.iter().enumerate() {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx < 0 || ny < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize {
                        valid = false;
                        break;
                    }

                    word[i] = map[ny as usize][nx as usize];
                }

                if valid && (&word == b"XMAS" || &word == b"SAMX") {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
