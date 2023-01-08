use std::fs;
use std::collections::HashSet;



fn find_position(line: &str) -> Result<usize, usize> {
    for i in 3..line.len() {
        let mut hs = HashSet::new();

        for x in 0..=3 {
            hs.insert(line.chars().nth(i - x).unwrap());
        }

        if hs.len() == 4 {
            return Ok(i+1);
        }
    }

    return Err(0);
}

fn main() {
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };


    for line in contents.lines() {
        println!("{} {}", find_position(line).unwrap(), line);
    }
}
