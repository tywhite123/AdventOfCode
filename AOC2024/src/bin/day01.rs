use std::fs;

fn main() {
    println!("Day 01:");
    
    let contents = fs::read_to_string("./inputs/input01.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    
    for x in split_values
    {
        let split: Vec<&str> = x.split(' ').collect();
        list1.push(split[0].parse::<i32>().unwrap());
        list2.push(split[3].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total : i32 = 0;
    for i in 0..list1.len()
    {
        let diff = list1[i] - list2[i];
        total += diff.abs();

    }


    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    
    for x in split_values
    {
        let split: Vec<&str> = x.split(' ').collect();
        list1.push(split[0].parse::<i32>().unwrap());
        list2.push(split[3].parse::<i32>().unwrap());
    }

    //list1.sort();
    //list2.sort();
    
    let mut total : i32 = 0;
    for value in list1
    {
        let instances = list2.iter().filter(|&n| *n == value).count() as i32;
        total += value*instances;
    }
    println!("Part 2 Answer = {}", total);
}