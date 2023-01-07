use std::{fs, collections::BTreeMap};


fn read_line(line: &str, dict: &mut BTreeMap::<usize, Vec<char>>) {

    let l = line.len();
    for col in (1..l).step_by(4) {
        let nr_stack = 1 + (col - 1) / 4;
        let val = line.chars().nth(col as usize).unwrap();

        if val == ' ' {
            continue;
        }

        let d = dict.entry(nr_stack).or_insert(Vec::new());
        d.insert(0, val);


        // println!("{} {} ", nr_stack, val);

    }

}

fn mov(line: &str, dict: &mut BTreeMap<usize, Vec<char>>) {
    let num = line.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
    let from = line.split(' ').nth(3).unwrap().parse::<usize>().unwrap();
    let to = line.split(' ').nth(5).unwrap().parse::<usize>().unwrap();

    for _ in 0..num {
        let tmp;
        {
            let f = dict.get_mut(&from).unwrap();
            tmp = f.pop().unwrap();
        }
        let t =  dict.get_mut(&to).unwrap();
        t.push(tmp);
    }
    println!("line = {} {} {}", num, from, to);
    // println!(" {:#?}", dict);
}

fn get_result(dict: BTreeMap::<usize, Vec<char>>) {
    for (_, v) in dict {
         print!("{}", v.last().unwrap());

    }

    println!("" );

}
fn main() {
    let mut dict: BTreeMap<usize, Vec::<char>> = BTreeMap::new();
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };


    let mut phase = 0;
    for line in contents.lines() {
        if line.trim().starts_with('1') {
            phase = 1;
            continue;
        }
        if line.trim().starts_with("move") {
            phase = 2;
        }

        if phase == 0 {
            // Get the stack
            read_line(line, &mut dict);
        } else if phase == 1 {
            println!(" dic = {:#?}", dict);
            // space only
            continue;
        } else if phase == 2 {
            mov(line, &mut dict);
        }
    }

    get_result(dict);

    // We have the initial stack in dict
    //

}
