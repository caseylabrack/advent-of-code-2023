use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("data/example2.txt").expect("couldn't read puzzle input, dummy");
    let data : Vec<&str> = data.lines().collect();
    
    let digitlike = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut sum = 0;

    for line in data {

        let digitlikes: Vec<&str> = digitlike
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();

        for d in &digitlikes {
            print!("{}",d);
        }
        println!("");


        let mut first = digitlikes[0];
        if first.len() > 1 {
            first = digitWordToDigit(first);
        }
        let mut last = digitlikes[digitlikes.len()-1];
        if last.len() > 1 {
            last = digitWordToDigit(last);
        }

        // println!(" {}{}", first,last);
        
        let mut num = first.to_string();
        num.push_str(last);
        
        sum += num.parse::<i32>().unwrap();
    }


    println!("sum {}", sum);

}

fn digitWordToDigit (s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => unreachable!()
    }
}