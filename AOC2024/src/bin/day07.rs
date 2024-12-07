use std::{fs, result};

use itertools::Itertools;

fn main() {
    println!("Day 07:");
    
    let contents = fs::read_to_string("./inputs/input07.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let operators = ['*', '+'];
    let split_values : Vec<&str> = contents.split('\n').collect();
    let info: Vec<(i64, Vec<i64>)> = split_values.iter().filter_map(|line|{
        let split_line: Vec<&str> = line.split(": ").collect();
        let split_values: Vec<i64> = split_line[1].split(" ").filter_map(|f| Some(f.parse::<i64>().unwrap())).collect();

        //println!("{}", split_line[0]);
        Some((split_line[0].parse::<i64>().unwrap(), split_values))
    }).collect();

    let mut total_true_equations = 0;
    for (total, values) in info {
        let gaps = values.len()-1;
        //let combinations = operators.iter().cloned().multi_cartesian_product(gaps);
        
        let combinations = (0..gaps)
        .map(|_| operators.iter().cloned())
        .multi_cartesian_product();
 
        
        for ops in  combinations{
            let mut result = values[0];
            for (i, & op) in ops.iter().enumerate()
            {
                match op {
                    '*' => result *= values[i + 1],
                    '+' => result += values[i + 1],
                    _ => result += 0,
                }
            }

            if result == total {
                total_true_equations += total;
                break;
            }
        }
    };

    println!("Part 1 Answer = {}", total_true_equations);
}

fn part2(contents: &str)
{
    let operators = ["*", "+", "||"];
    let split_values : Vec<&str> = contents.split('\n').collect();
    let info: Vec<(i64, Vec<i64>)> = split_values.iter().filter_map(|line|{
        let split_line: Vec<&str> = line.split(": ").collect();
        let split_values: Vec<i64> = split_line[1].split(" ").filter_map(|f| Some(f.parse::<i64>().unwrap())).collect();

        //println!("{}", split_line[0]);
        Some((split_line[0].parse::<i64>().unwrap(), split_values))
    }).collect();

    let mut total_true_equations = 0;
    for (total, values) in info {
        let gaps = values.len()-1;
        //let combinations = operators.iter().cloned().multi_cartesian_product(gaps);
        
        let combinations = (0..gaps)
        .map(|_| operators.iter().cloned())
        .multi_cartesian_product();
 
        
        for ops in  combinations{
            let mut result = values[0];
            for (i, & op) in ops.iter().enumerate()
            {
                match op {
                    "*" => result *= values[i + 1],
                    "+" => result += values[i + 1],
                    "||" => result = concat_int(result, values[i + 1]), 
                    _ => result += 0,
                }
            }

            if result == total {
                //println!("{}", total);
                total_true_equations += total;
                break;
            }
        }
    };

    println!("Part 2 Answer = {}", total_true_equations);
}

fn concat_int(int_a: i64, int_b: i64) -> i64 {
    let a = int_a.to_string();
    let b = int_b.to_string();

    let result = (a + &b).parse::<i64>().unwrap();
    //println!("{} {} {}", int_a, int_b, result);
    
    return result;
}