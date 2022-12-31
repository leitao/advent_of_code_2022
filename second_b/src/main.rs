use std::fs;

// A X = Rock = 1
// B Y = Paper 2
// C Z = Scisor 3

#[derive(Copy, Clone)]
enum Shapes {
    ROCK = 1,
    PAPER = 2,
    SCISOR = 3,
}

impl Shapes{
    fn get(shape: &str) -> Shapes {
        match shape {
            "A" => Shapes::ROCK,
            "B" => Shapes::PAPER,
            "C" => Shapes::SCISOR,
            _  => panic!(),
        }
    }
}


#[derive(Copy, Clone)]
enum Results {
    LOSS = 0,
    DRAW = 3,
    WIN = 6,
}

impl Results {
    fn get(shape: &str) -> Results {
        match shape {
            "X" => Results::LOSS,
            "Y" => Results::DRAW,
            "Z" => Results::WIN,
            _  => panic!(),
        }
    }
}


fn against_rock(result: Results) -> Shapes {
    match result {
        Results::LOSS => Shapes::SCISOR,
        Results::WIN => Shapes::PAPER,
        Results::DRAW => Shapes::ROCK,
    }
}

fn against_paper(result: Results) -> Shapes {
    match result {
        Results::LOSS => Shapes::ROCK,
        Results::WIN => Shapes::SCISOR,
        Results::DRAW => Shapes::PAPER,
    }
}

fn against_scisor(result: Results) -> Shapes {
    match result {
        Results::LOSS => Shapes::PAPER,
        Results::WIN => Shapes::ROCK,
        Results::DRAW => Shapes::SCISOR,
    }
}

fn shape_mine(other: Shapes, result: Results) -> Shapes {
    match other {
        Shapes::ROCK => against_rock(result),
        Shapes::PAPER => against_paper(result),
        Shapes::SCISOR => against_scisor(result),
    }
}


fn get_results(mine: Shapes, result: Results) -> i32 {
    let mut sum = 0;

    sum +=  match mine {
       Shapes::ROCK => 1,
       Shapes::PAPER => 2,
       Shapes:: SCISOR => 3,
    };

    sum += match result {
        Results::WIN => 6,
        Results::LOSS => 0,
        Results::DRAW => 3,
    };

    return sum;
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
        // println!("{:?}",vec);
        let other = Shapes::get(vec[0]);
        let dresult = Results::get(vec[1]);

        let mine = shape_mine(other, dresult.clone());



        sum += get_results(mine, dresult);
        println!(" after = {}\n", sum);

    }

    println!("Sum is {}", sum);
}
