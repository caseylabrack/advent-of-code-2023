use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("couldn't read puzzle input, dummy");
    let data : Vec<&str> = data.lines().collect();
    
    let digit = Regex::new(r"[0-9]").unwrap();
    let digitlike = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    println!("part 1: {}", solve(digit, &data));
    println!("part 2: {}", solve(digitlike, &data));
}

fn solve (test: Regex, lines : &Vec<&str>) -> i32 {
    let mut sum = 0;

    for line in lines {

        let mut first = match test.find(line) {
            Some(m) => m.as_str(),
            None => panic!("no digit in this line? {}", line)
        };
        if first.len() > 1 {
            first = digit_word_to_digit(first);
        }

        let mut last = regex_last(&test, line).unwrap();
        if last.len() > 1 {
            last = digit_word_to_digit(last);
        }

        let mut num = first.to_string();
        num.push_str(last);
        
        sum += num.parse::<i32>().unwrap();
    }

    return sum;
}

fn digit_word_to_digit (s: &str) -> &str {
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
        _ => panic!("{} is not a digit word?", s)
    }
}

// a regex that does `find` starting from the back
// because regex crate can't find overlapping matches, ie "oneight" only catches "one", not "eight"  
fn regex_last<'a> (test : &Regex, hay: &'a str) -> Option<&'a str> {
    let mut last = None;
    for idx in (0..hay.len()).rev() {
        match test.find_at(hay, idx) {
            Some(c) => {
                last = Some(c.as_str());
                break;              
            },
            None => ()
        }
    }
    return last;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digit_word () {
        assert_eq!(digit_word_to_digit("one"), "1");
        assert_eq!(digit_word_to_digit("nine"), "9");
    }

    #[test]
    #[should_panic]
    fn bad_digit_word () {
        let _ = digit_word_to_digit("yeet");
    }
    
    #[test]
    fn overlapping_matches () {
        let t = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        assert_eq!(regex_last(&t, "oneight"), Some("eight"));
    }
}