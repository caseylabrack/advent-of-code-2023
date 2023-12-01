use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("couldn't read puzzle input, dummy");
    let data : Vec<&str> = data.lines().collect();
    
    let is_digit = Regex::new(r"[0-9]").unwrap();
    let mut sum = 0;

    for line in data {

        let digit_chars: Vec<&str> = is_digit
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();

        let mut num = digit_chars.get(0).unwrap().to_string();
        num.push_str(digit_chars.last().unwrap());
        sum += num.parse::<i32>().unwrap(); 
        println!("{}", num);
    }

    println!("sum {}", sum);

}

fn firstAndLastDigitsHard (s:&str) {
    let as_chars = s.chars().collect::<Vec<char>>();

    // for char in 
}
