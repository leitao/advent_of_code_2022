use std::fs;


fn main() {
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };

    let mut max = 0;
    for line in contents.lines() {
        // println!(" {}", line);
        let l = line.len() / 2;
        let first: Vec<char> = line[0..l].chars().collect();
        let second: Vec<char> = line[l..].chars().collect();


        let mut common: Vec::<char> = Vec::new();

        for c in first {
            if second.contains(&c) && !common.contains(&c){
                common.push(c);
            }
        }

        for i in common.iter() {
            let val = *i as i32 - 'a' as i32 + 1;
            let mut f = val;

            if val < 0 {
                f = *i as i32 - 'A' as i32 + 27;
            }
            println!("c = {} {}", *i, f);

            max += f;
        }

    }
    println!("Max = {}", max);
}
