use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    println!("Day 08:");
    
    let contents = fs::read_to_string("./inputs/input8.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let mut lines = contents.lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let instructions = lines.next().unwrap();
    lines.next();
    for l in lines
    {
        let splits: Vec<&str> = l.split(" = ").collect();
        let directions: (&str, &str) = splits[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        map.insert(splits[0], directions);
    }

    let no_of_steps = find_steps("AAA", instructions, &map);

    println!("Part 1 Answer = {no_of_steps}");
}

fn part2(contents: &str)
{
    let mut lines = contents.lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"([A-Z]{2}[A])").unwrap();
    let mut starts: Vec<&str> = Vec::new();

    let instructions = lines.next().unwrap();
    lines.next();
    for l in lines
    {
        let splits: Vec<&str> = l.split(" = ").collect();
        if re.is_match(splits[0])
        {
            starts.push(splits[0]);
        }
        let directions: (&str, &str) = splits[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        map.insert(splits[0], directions);
    }

    let no_of_steps_arr: Vec<i32> = starts.iter().map(|x| find_steps(*x, instructions, &map)).collect();
    let mut no_of_steps: i64 = no_of_steps_arr[0] as i64;
    for n in 0..no_of_steps_arr.len()
    {
        no_of_steps = lcm(no_of_steps, no_of_steps_arr[n] as i64);
    }

    println!("Part 2 Answer = {no_of_steps}");
}

fn find_steps(start: &str, instructions: &str, map: &HashMap<&str, (&str, &str)>) -> i32
{

    let mut no_of_steps = 0;
    let mut current_instruction = 0;
    let mut current = start;

    let end_re = Regex::new(r"([A-Z]{2}[Z])").unwrap();
    while !end_re.is_match(current)
    {
        if current_instruction >= instructions.len()
        {
            current_instruction = 0;
        }
        let next_step = instructions.chars().nth(current_instruction).unwrap();
        current_instruction+=1; 

        if next_step == 'L'
        {
            current = map.get(current).unwrap().0;
        }
        else
        {
            current = map.get(current).unwrap().1;
        }

        no_of_steps += 1;
    }
    return no_of_steps;
}

fn gcd(first: i64, second: i64) -> i64
{
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(first: i64, second: i64) -> i64
{
    return first*second/gcd(first, second);
}