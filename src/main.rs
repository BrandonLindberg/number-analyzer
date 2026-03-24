use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};

fn main() {
    let filename = "numbers.txt";
    let numbers = get_user_number();

    write_file(filename, &numbers);

    let read_numbers = read_file(filename);

    println!("Retrieving numbers from file...\n");

    for num in &read_numbers {
        check_number(num);
    }
}

fn get_user_number() -> Vec<i32> {
    let mut numbers = Vec::new();

    loop {
        println!("Enter a number; Type 'done' to finish:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        match input {
            "done" => break,
            _ => {
                match input.parse::<i32>() {
                    Ok(num) => numbers.push(num),
                    Err(_) => println!("Invalid input"),
                }
            }
        }
    }
    numbers
}

fn write_file(filename: &str, numbers: &Vec<i32>) {
    let mut file = File::create(filename).expect("Could not create file");

    for num in numbers {
        writeln!(file, "{}", num).expect("Could not write to file");
    }
}

fn read_file(filename: &str) -> Vec<i32> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        numbers.push(num);
    }

    numbers
}

fn check_number(num: &i32) {
    match num {
        n if *n > 0 => println!("{} is positive", n),
        n if *n < 0 => println!("{} is negative", n),
        _ => println!("{} is zero", num),
    }
}