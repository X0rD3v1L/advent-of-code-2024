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

    for x in 0..map[0].len() as isize {
        for y in 0..map.len() as isize {
            let coords = [
                (x + 1, y + 1), 
                (x, y),         
                (x, y + 2),     
                (x + 2, y),     
                (x + 2, y + 2), 
            ];

            let mut valid = true;
            let mut cross_pattern = [0; 4];
            let mut iter = coords.iter().map(|&(cx, cy)| {
                map.get(cy as usize)
                    .and_then(|row| row.get(cx as usize).copied())
                    .unwrap_or_default()
            });

            if iter.next().unwrap_or_default() != b'A' {
                valid = false;
            }

            if valid {
                cross_pattern[0] = iter.next().unwrap_or_default();
                cross_pattern[1] = iter.next().unwrap_or_default();
                cross_pattern[2] = iter.next().unwrap_or_default();
                cross_pattern[3] = iter.next().unwrap_or_default();

                if &cross_pattern == b"MMSS"
                    || &cross_pattern == b"MSMS"
                    || &cross_pattern == b"SSMM"
                    || &cross_pattern == b"SMSM"
                {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
