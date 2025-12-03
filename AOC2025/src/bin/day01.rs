use std::fs;

fn main() {
    println!("Day 01:");
    
    let contents = fs::read_to_string("./inputs/input01.txt").expect("Not Found");

    //println!("{}", contents);

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
   const dial_start: i32 = 50; 

    let split_values : Vec<&str> = contents.split('\n').collect();

    //println!("");

    let parts : Vec<(&str, i32)> = split_values.iter().map(|f| {
        let val : (&str, &str) = f.split_at(1);

        let num : i32 = val.1.parse::<i32>().unwrap();

        (val.0, num)
    }).collect(); 

    let mut current_dial_val: i32 = dial_start;
    let mut zeros: i32 = 0;

    for part in parts {
        let mut val_to_mod = current_dial_val;

        if part.0 == "L" {
            val_to_mod = val_to_mod - part.1;
        }
        else {
            val_to_mod = val_to_mod + part.1;
        }

        current_dial_val = modulo(val_to_mod, 100);
        if current_dial_val == 0 {
            zeros = zeros + 1;
        }
    }


    //println!("{}", current_dial_val);

    
    println!("Part 1 Answer = {}", zeros);
}

fn part2(contents: &str)
{
   const dial_start: i32 = 50; 

    let split_values : Vec<&str> = contents.split('\n').collect();

    //println!("");

    let parts : Vec<(&str, i32)> = split_values.iter().map(|f| {
        let val : (&str, &str) = f.split_at(1);

        let num : i32 = val.1.parse::<i32>().unwrap();

        (val.0, num)
    }).collect(); 

    let mut current_dial_val: i32 = dial_start;
    let mut zeros: i32 = 0;

    for part in parts {
        for i in 0..part.1
        {
            if part.0 == "L" {
                current_dial_val -= 1;
            }
            else {
                current_dial_val += 1;
            }

            current_dial_val = modulo(current_dial_val, 100);
            if current_dial_val == 0
            {
                zeros += 1;
            }
        }

    }

    println!("Part 2 Answer = {}", zeros);
}

fn modulo(a: i32, b: i32) -> i32
{
    return ((a % b) + b) % b;
}