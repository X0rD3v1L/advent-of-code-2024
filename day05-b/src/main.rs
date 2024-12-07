use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("src/input.txt");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut ordering: Vec<(i32, i32)> = Vec::new();
    let mut books: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let line = line?;
        if !line.is_empty() && line.contains('|'){
            let nums: Vec<i32> = line
                .split('|')
                .map(|s| s.trim().parse().expect("Failed to parse number"))
                .collect();
            ordering.push((nums[0], nums[1]));
        } else if !line.is_empty() && line.contains(','){
            if !line.is_empty() && line.contains(',') {
                let book: Vec<i32> = line
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                books.push(book);
            }
        }
    }

    println!("{:?}", solve(&ordering, &mut books));

    Ok(())
}

fn solve(ordering: &[(i32, i32)], books: &mut [Vec<i32>]) -> i32 {
    let result: i32 = books
    .iter_mut()
    .filter(|book| {
        !book.is_sorted_by(|&a, &b| !ordering.contains(&(b, a)))
    })
    .map(|book| {
        book.sort_by(|&a, &b| ordering.contains(&(b, a)).cmp(&true));
        book[book.len() / 2]
    })
    .sum();

    return result;
}
