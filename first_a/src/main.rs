use std::fs;

fn main() {
    let mut max: u64 = 0;

    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };


    let mut tmp: u64 = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            println!("new");
            if tmp > max {
                max = tmp;
            }
            tmp = 0;
            continue;
        }

        // println!("line = {}", line);

        tmp += line.parse::<u64>().unwrap();
    }

    println!("Max is {}", max);
}
