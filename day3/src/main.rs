use std::{fs, i32};

fn main() {
    let data = fs::read_to_string("data/example.txt").expect("couldn't read puzzle input, dummy");

    let data: Vec<&str> = data.lines().collect();

    

    // let grid = CharGrid::new(&data);

}

struct CharGrid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl CharGrid {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.data.get(y * self.width + x)
    }

    fn new(s: &str) -> Self {

        let d = s
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .collect::<Vec<char>>();

        let data = s.lines().collect::<Vec<&str>>();
        let w = data[0].len();
        let h = data.len();

        CharGrid { data: d, width: w, height: h }
    }

    fn has_symbol_adjacent (&self, x:usize, y: usize) -> bool {
        for i in -1..=1 {
            for j in -1..=1 {
                let neighbor = self.get(x + i as usize, y + j as usize);
                // on the grid?
                let neighbor = match neighbor {
                    Some(c) => *c,
                    None => return false
                };
                // symbol?
                if !neighbor.is_ascii_digit() && neighbor != '.' { return true }
            }
        }

        false
    }

    // fn number_from_index(&self, x: usize, y: usize)-> Option<i32> {
        
    //     let value = match self.get(x,y) {
    //         Some(e) => *e,
    //         None => return None
    //     };

    //     let digit = match value.to_digit(10) {
    //         Some(e) => e as i32,
    //         None => return None
    //     };
        
    //     let mut left_nums: Vec<char> = vec![];
    //     let mut x2 = x;
    //     loop {
    //         x2-=1;
    //         let left = self.get(x2,y); 

    //         // left the grid
    //         if left.is_none() { break } ;

    //         // is actually a digit
    //         let left = *left.unwrap();
    //         if left.is_ascii_digit() { break };

    //         left_nums.push(left);
    //     }

    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_into_scheme () {
        let grid: CharGrid = CharGrid::new("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

        assert_eq!(grid.get(0, 0), Some(&'4'));
        assert_eq!(grid.get(1, 1), Some(&'.'));
        assert_eq!(grid.get(2, 2), Some(&'3'));
        assert_eq!(grid.get(7, 9), Some(&'8'));
    }

    #[test]
    fn bad_index () {
        let grid: CharGrid = CharGrid::new("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        
        assert_eq!(grid.get(10, 9), None);
        assert_eq!(grid.get(9, 10), None);
    }

    #[test]
    fn adjacent_dig () {
        let grid: CharGrid = CharGrid::new("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        
        assert!(grid.has_symbol_adjacent(3, 2));
    }

    

    // fn get_num_from_cell () {
    //     let grid: CharGrid = CharGrid::new("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

    //     assert_eq!(grid.number_from_index(1, 0), Some(467));
    // }
}