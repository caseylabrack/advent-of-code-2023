use std::{fs, str::FromStr};
use regex::Regex;

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("couldn't read puzzle input, dummy");
    let data : Vec<&str> = data.lines().collect();

    let games = data
        .iter()
        .map(|s| s.parse::<Game>().unwrap());

    let possible_games_part1:i32 = games
        .filter(|g| g.r <= 12 && g.g <= 13 && g.b <=14)
        .map(|g| g.id)
        .sum();

    let all_games_power_part2:i32 = data
        .iter()
        .map(|s| s.parse::<Game>().unwrap())
        .map(|g| g.r * g.g * g.b)
        .sum();

    println!("part1: {}", possible_games_part1);
    println!("part2: {}", all_games_power_part2);
}

#[derive(Debug, PartialEq, )]
struct Game {
    r: i32,
    g: i32,
    b: i32,
    id: i32
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let find_id = Regex::new(r"Game (\d+):").expect("badly formed game id regex");
        let captures_id = find_id.captures(s).expect("couldn't find game id pattern");
        let id = match captures_id.get(1) {
            Some(m) => m.as_str(),
            None => return Err("couldn't find id")
        };
        let id:i32 = match id.parse() {
            Ok(e) => e,
            Err(_) => return Err("id was not a number")
        };

        Ok(Game { r: max_number_of_color(s, Color::Red), g: max_number_of_color(s, Color::Green), b: max_number_of_color(s, Color::Blue), id: id })
    }
}

fn max_number_of_color (s:&str, color: Color) -> i32 {

    let pattern = match color {
        Color::Red => r"(\d+) red",
        Color::Green => r"(\d+) green",
        Color::Blue => r"(\d+) blue"
    };
    let find_color = Regex::new(pattern).expect("badly formed cube regex");
    let mut cubes: Vec<i32> = vec![];
    for (_, [n]) in find_color.captures_iter(s).map(|c| c.extract()) {
        cubes.push(n.parse().unwrap());
    }

    *cubes.iter().max().unwrap()
}

enum Color {
    Red,
    Green,
    Blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game () {
        let g = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(g.parse(), Ok(Game {r: 20, g: 13, b: 6, id: 3}));

        let g = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(g.parse(), Ok(Game {r: 14, g: 3, b: 15, id: 4}));
    }

    #[test]
    fn parse_color_cubes () {
        assert_eq!(max_number_of_color("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Color::Red), 20);
        assert_eq!(max_number_of_color("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Color::Blue), 15);
    }
}