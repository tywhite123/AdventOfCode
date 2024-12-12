use std::fs;

struct Plot {
    plant: char,
    position: (i32,i32)
}

struct Region {
    plots: Vec<Plot>
}

fn main() {
    println!("Day 12:");
    
    let contents = fs::read_to_string("./inputs/input6.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(_contents: &str)
{
    //let directions = [(-1,0), (1, 0), (0, -1), (0 , 1)];

    //let mut unique_chars: HashSet<char> = HashSet::new();

    //let split: Vec<(char, (i32, i32))> = contents
    //    .lines()
    //    .enumerate()
    //    .flat_map(|(y, f)| f.chars()
    //        .enumerate()
    //        .map(move |(x, f)| 
    //            (f, (x as i32, y as i32))
    //        )).collect();

    //for (c, _) in split.iter() {
    //    unique_chars.insert(*c);
    //}

    //for char in &unique_chars{
    //    let positions: Vec<(i32,i32)> = split
    //        .iter()
    //        .filter(|(c,_)| char == c)
    //        .map(|(_, pos)| *pos)
    //        .collect();

    //    let area = positions.len() as i32;


    //}


    println!("Part 1 Answer = ");
}

fn part2(_contents: &str)
{
    println!("Part 2 Answer = ");
}
