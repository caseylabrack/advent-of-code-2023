use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("couldn't read puzzle input, dummy");

    let data: Vec<&str> = data.lines().collect();

    let find_number = Regex::new("[0-9]+").unwrap();
    let find_symbol = Regex::new(r"[^\d\.]").unwrap(); // not a digit or a dot
    let find_gear = Regex::new(r"\*").unwrap();

    // find a gear, add it to the hashmap
    // key: x,y position tuple of the gear
    // value: the part number 
    // upon key collision, you have both part numbers that are connected to the gear
    let mut gears: HashMap<(usize, usize), i32> = HashMap::new();

    let mut parts_sum = 0;
    let mut gear_ratios_sum = 0;

    for (row, line) in data.iter().enumerate() {
        for m in find_number.find_iter(line) {
            
            let part_num = m.as_str().parse::<i32>().unwrap();

            // find the range in which to scan for symbols
            let start_scan = match m.range().start {
                0 => 0,
                _ => m.range().start - 1
            };
            let scan_width = m.range().end + 1 - start_scan;

            // scan line and neighbor lines. 
            // collect all chars in a string
            // once all neighbors to the part number are in string, search for a symbol (part 1)  
            let mut neighbor_chars = "".to_string();

            // scan this line
            let this_line = line.chars().skip(start_scan).take(scan_width).collect::<String>();
            neighbor_chars.push_str(this_line.as_str());
            if let Some(gear) = find_gear.find(&this_line) { // find a gear?
                if let Some(previous_part_num) = gears.insert((start_scan + gear.range().start, row), part_num) { // insert gear and check for key collision
                    gear_ratios_sum += previous_part_num * part_num;
                }
            }
            
            // scan line below
            if let Some(below) = data.get(row + 1) {
                let below_line = below.chars().skip(start_scan).take(scan_width).collect::<String>();
                neighbor_chars.push_str(below_line.as_str());
                if let Some(gear) = find_gear.find(&below_line) { // find a gear?
                    if let Some(previous_part_num) = gears.insert((start_scan + gear.range().start, row + 1), part_num) { // insert gear and check for key collision
                        gear_ratios_sum += previous_part_num * part_num;
                    }
                }
            }

            // scan line above
            if row > 0 {
                if let Some(above) = data.get(row - 1) {
                    let above_line = above.chars().skip(start_scan).take(scan_width).collect::<String>();
                    neighbor_chars.push_str(above_line.as_str());
                    if let Some(gear) = find_gear.find(&above_line) { // find a gear?
                        if let Some(previous_part_num) = gears.insert((start_scan + gear.range().start, row - 1), part_num) { // insert gear and check for key collision
                            gear_ratios_sum += previous_part_num * part_num;
                        }
                    }
                }
            }
            if find_symbol.is_match(neighbor_chars.as_str()) {
                parts_sum += part_num;
            }
        }
    }

    println!("part numbers sum: {}", parts_sum);
    println!("gear ratios sum: {}", gear_ratios_sum);
}