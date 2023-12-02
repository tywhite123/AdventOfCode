use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("../inputs/input1.txt").expect("Test");
    part_1(&contents);
    part_2(&contents);
    //println!("{contents}");
}

fn part_1(contents: &str){
    let split_values : Vec<&str> = contents.split('\n').collect();
    
    let mut total: i32 = 0;
    let re = Regex::new(r"\d{1}").unwrap();
    for value in split_values
    {
        
        let digits: Vec<&str> = re.find_iter(value).map(|n| n.as_str()).collect();
        if digits.is_empty(){
            continue;
        }

        let combined = format!("{}{}", digits[0], digits.last().unwrap());
        total += combined.parse::<i32>().unwrap();

    }

    println!("Part 1: Total = {}", total);
}


fn part_2(contents: &str){
    let replacements = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("eighthree", "83"),
        ("eightwo", "82"),
        ("fiveight", "58"),
        ("nineight", "98"),
        ("sevenine", "79"),
    ]);


    let split_values : Vec<&str> = contents.split('\n').collect();
    //:println!("{}", split_values[0]);
    

    let mut total: i32 = 0;
    let re = Regex::new(r"\d{1}").unwrap();
    for value in split_values
    {
        let mut string = value.to_string();

        for (key, value) in replacements.iter(){ 
            if string.contains(key){
                string = string.replace(key, value); 
            }
        }


        let digits: Vec<&str> = re.find_iter(string.as_str()).map(|n| n.as_str()).collect();
        if digits.is_empty(){
            continue;
        }
//        println!("{}{}", digits[0], digits.last().unwrap());


        let combined = format!("{}{}", digits[0], digits.last().unwrap());
        total += combined.parse::<i32>().unwrap();

    }


    println!("Part 2: Total = {}", total);
}
