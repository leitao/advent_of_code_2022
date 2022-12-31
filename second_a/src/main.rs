use std::fs;

// A X = Rock = 1
// B Y = Paper 2
// C Z = Scisor 3

enum Shapes {
    ROCK = 1,
    PAPER = 2,
    SCISOR = 3,
}

enum Results {
    LOSS = 0,
    DRAW = 3,
    WIN = 6,
}

fn shape_mine(shape:&str) -> i8 {
    match shape {
        "X" => Shapes::ROCK as i8,
        "Y" => Shapes::PAPER as i8,
        "Z" => Shapes::SCISOR as i8,
        _ => 0,
    }
}

fn get_a(mine: &str) -> i8 {
    match mine {
        "X" => Results::DRAW as i8,
        "Y" => Results::WIN as i8,
        "Z" => Results::LOSS as i8,
        _ => 0,
    }
}

fn get_b(mine: &str) -> i8 {
    match mine {
        "X" => Results::LOSS as i8,
        "Y" => Results::DRAW as i8,
        "Z" => Results::WIN as i8,
        _ => 0,
    }
}

fn get_c(mine: &str) -> i8 {
    match mine {
        "X" => Results::WIN as i8,
        "Y" => Results::LOSS as i8,
        "Z" => Results::DRAW as i8,
        _ => 0,
    }
}


fn result(other: &str, mine: &str) -> i8 {
    match other {
        "A" => get_a(mine),
        "B" => get_b(mine),
        "C" => get_c(mine),
        _ => 0,
    }
}


fn main() {
    let mut sum : i32 = 0;
    let file_path = "input.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(n) => n,
        Err(_) => String::from(""),
    };

    for line in contents.lines() {
        let vec : Vec<&str>  = line.split(" ").collect();
        println!("{:?}",vec);
        let other = vec[0];
        let mine = vec[1];

        sum += shape_mine(mine) as i32;
        println!(" mine = {}\n", sum);
        sum += result(other, mine) as i32;
        println!(" after = {}\n", sum);

    }

    println!("Sum is {}", sum);
}
