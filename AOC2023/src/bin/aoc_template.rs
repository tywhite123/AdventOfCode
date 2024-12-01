use std::fs;

fn main() {
    println!("Template:");
    
    let contents = fs::read_to_string("./inputs/input6.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(_contents: &str)
{
    println!("Part 1 Answer = ");
}

fn part2(_contents: &str)
{
    println!("Part 2 Answer = ");
}