use std::fs;

fn main() {
    let mut vec = Vec::new();

    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };


    let mut tmp: u64 = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            vec.push(tmp);
            tmp = 0;
            continue;
        }

        // println!("line = {}", line);

        tmp += line.parse::<u64>().unwrap();
    }

    vec.sort();
    vec.reverse();

    // let slice = vec.as_slice()[0..3];
    let slice = &vec[0..3];

    println!("Slice = {:?}", slice.iter().sum::<u64>());
}
