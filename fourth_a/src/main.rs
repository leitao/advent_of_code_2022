use std::fs;

fn get_end(range: &str) -> i32 {
    let mut x = range.split("-");

    // Discarding the first entry
    x.next();

    let first =  match x.next() {
        None => panic!("Not able to parse"),
        Some(z) => z,
    };

    match first.parse(){
        Ok(n) => n,
        Err(n) => panic!("{}", n)
    }
}

fn get_start(range: &str) -> i32 {
    let mut x = range.split("-");

    let first =  match x.next() {
        None => panic!("Not able to parse"),
        Some(z) => z,
    };

    match first.parse(){
        Ok(n) => n,
        Err(n) => panic!("{}", n)
    }
}

fn main() {
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };


    let mut count = 0;
    for line in contents.lines() {
        let tuple: Vec<&str>  = line.split(",").collect();
        let a_entry = tuple[0];
        let b_entry = tuple[1];

        // println!("{} {}", a_entry, b_entry);
        let a_start = get_start(a_entry);
        let a_end = get_end(a_entry);
        // println!("a {} - {}", a_start, a_end);

        let b_start = get_start(b_entry);
        let b_end = get_end(b_entry);
        // println!("b {} - {}\n", b_start, b_end);

        // a is inside b
        println!(" Line = {}", line);
        if a_start >= b_start && b_end >= a_end{
            println!("A is inside B a:{} b:{} ", a_entry, b_entry);
            count+= 1;
            // We only count a pair once
            continue;
        }

        // b  is inside a
        if b_start >= a_start && a_end >= b_end{
            println!("B is inside A a:{} b:{} ", a_entry, b_entry);
            count+=1;
        }
        println!();
    }

    println!("total is {:?}", count);

}
