use std::fs;
use regex::{Regex, Match};

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("couldn't read puzzle input, dummy");

    let data: Vec<&str> = data.lines().collect();

    let find_number = Regex::new("[0-9]+").unwrap();
    let find_symbol = Regex::new(r"[^\d\.]").unwrap(); // not a digit or a dot

    let mut sum = 0;

    for (row, line) in data.iter().enumerate() {
        for m in find_number.find_iter(line) {
            
            // find the range in which to scan for symbols
            let start_scan = match m.range().start {
                0 => 0,
                _ => m.range().start - 1
            };
            let scan_width = m.range().end + 1 - start_scan;

            // scan line and neighbor lines. collect all chars in a string  
            let mut neighbor_chars = "".to_string();

            neighbor_chars.push_str(line.chars().skip(start_scan).take(scan_width).collect::<String>().as_str());
            
            if let Some(below) = data.get(row + 1) {
                neighbor_chars.push_str(below.chars().skip(start_scan).take(scan_width).collect::<String>().as_str());
            }
            if row > 0 {
                if let Some(above) = data.get(row - 1) {
                    // println!("{}: {}", row, l);
                    neighbor_chars.push_str(above.chars().skip(start_scan).take(scan_width).collect::<String>().as_str());
                }    
            }
            if find_symbol.is_match(neighbor_chars.as_str()) == true {
                sum += m.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("sum: {}", sum);
}