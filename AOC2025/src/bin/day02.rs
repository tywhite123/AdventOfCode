use std::fs;
use fancy_regex::Regex;

fn main() {
    println!("Day 02:");
    
    let contents = fs::read_to_string("./inputs/input02.txt").expect("Not Found");

    //part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let id_ranges : Vec<(i64, i64)> = contents.replace("\n","").split(',').map(|f| {
        let ids: Vec<i64> = f.split('-').map(|c| c.parse::<i64>().unwrap()).collect();

        (ids[0], ids[1])
    }).collect();

    let mut total = 0;

    for (start_id, end_id) in id_ranges {
        for i in start_id..=end_id {
            let string: String = i.to_string();
            let matches = Regex::new(r"\b([0-9]+)\1\b").unwrap().is_match(&string).unwrap();

            if matches {
                //println!("{}", string);
                total += i;
            }
        }
    }

    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let id_ranges : Vec<(i64, i64)> = contents.replace("\n","").split(',').map(|f| {
        let ids: Vec<i64> = f.split('-').map(|c| c.parse::<i64>().unwrap()).collect();

        (ids[0], ids[1])
    }).collect();

    let mut total = 0;

    for (start_id, end_id) in id_ranges {
        for i in start_id..=end_id {
            let string: String = i.to_string();
            let matches = Regex::new(r"\b([0-9]+)\1+\b").unwrap().is_match(&string).unwrap();

            if matches {
                //println!("{}", string);
                total += i;
            }
        }
    }
    println!("Part 2 Answer = {}", total);
}