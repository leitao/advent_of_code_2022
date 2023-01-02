use std::fs;


fn value(i: char) -> i32 {
    let val = i as i32 - 'a' as i32;
    let f;

    if val < 0 {
        f = i as i32 - 'A' as i32 + 26 + 1;
    } else {
        f = i as i32 - 'a' as i32 + 1;
    }
    println!("common = {} ({})", i, f);

    return f
}

// Brute force search
fn find_common(a: &str, b: &str, c: &str) -> char {
    for x in a.chars() {
        for y in  b.chars() {
            for z in c.chars() {
                // println!("{} {} {}", x, y, z);
                if x == y && y == z {
                    return x
                }
            }
        }
    }
    panic!("Not found");
}

fn main() {
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };

    let mut i = 0;
    let mut collect: Vec<Vec<&str>> = Vec::new();
    let mut tmp: Vec<&str> = Vec::new();

    for line in contents.lines() {
        tmp.push(line);
        // print!(" {:?}", tmp);
        if i == 2 {
            i = 0;
            assert!(tmp.len() == 3, "invalid vector size{}", tmp.len());
            collect.push(tmp.clone());
            tmp = Vec::new();
        } else {
            i += 1;
        }
    }


    println!("Group len = {}", collect.len());
    // println!("collect = {:#?}", collect);
    let mut sum = 0;
    for group in collect {
        println!("{:#?}", group);
        let common = find_common(group[0], group[1], group[2]);
        // println!("Common = {}", common);
        sum += value(common);
    }


    println!("Sum is {}", sum);
    // println!("collections = {:#?}", collect);
}
