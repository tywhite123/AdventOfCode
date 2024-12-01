use std::fs;

fn main() {
    println!("Day 09:");
    
    let contents = fs::read_to_string("./inputs/input9.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let lines = contents.lines();
    let mut total = 0;

    for l in lines
    {
        let line_array: Vec<i32> = l.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut arrays: Vec<Vec<i32>> = Vec::new();
        arrays.push(line_array);

        recursive_base_diff(&mut arrays, 0);

        let mut increase = 0;

        arrays.iter().rev().for_each(|l| increase = l[l.len()-1] + increase);

        total+=increase;
    }
    println!("Part 1 Answer = {total}");
}

fn part2(contents: &str)
{
    let lines = contents.lines();
    let mut total = 0;

    for l in lines
    {
        let line_array: Vec<i32> = l.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut arrays: Vec<Vec<i32>> = Vec::new();
        arrays.push(line_array);

        recursive_base_diff(&mut arrays, 0);

        let mut increase = 0;

        arrays.iter().rev().for_each(|l| increase = l[0] - increase);

        total+=increase;
    }

    println!("Part 2 Answer = {total}");
}

fn recursive_base_diff(diff: &mut Vec<Vec<i32>>, level: usize)
{
    diff.push(Vec::new());
    for i in 0..diff[level].len()-1
    {
        let x = diff[level][i+1];
        let y = diff[level][i];
        diff[level+1].push(x-y);
    }

    match diff[level+1].iter().min() == diff[level+1].iter().max() {
        true => return,
        false => recursive_base_diff(diff, level+1)
    }
}