use std::fs;
use regex::Regex;

fn main() {
    println!("Day 03:");
    
    let contents = fs::read_to_string("./inputs/input03.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let values: Vec<(i32, i32)> = regex.find_iter(contents)
        .map(|n| {
            let inner = n.as_str()
            .trim_start_matches("mul(")
            .trim_end_matches(")")
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        (inner[0], inner[1])
        })    
        .collect();

    let total = values.iter().fold(0, | sum, &n| sum + (n.0 * n.1));

    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let regex = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do(n't)?\(\))").unwrap();
    let mut do_save = true;
    let values: Vec<(i32, i32)> = regex.find_iter(contents)
        .filter_map(|n| {
            match n.as_str() {
                "do()" => {
                    do_save = true;
                    None
                }
                "don't()" => {
                    do_save = false;
                    None
                }
                _ if do_save => {
                    let inner = n.as_str()
                    .trim_start_matches("mul(")
                    .trim_end_matches(")")
                    .split(',')
                    .map(|f| f.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                    Some((inner[0], inner[1])) 
                }
                _ => None
            }
        })
        .collect();

    let total = values.iter().fold(0, | sum, &n| sum + (n.0 * n.1));
    println!("Part 2 Answer = {}", total);
}