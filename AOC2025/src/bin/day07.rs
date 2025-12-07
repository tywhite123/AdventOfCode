use std::{collections::{BTreeSet, HashMap}, fs};

fn main() {
    println!("Day07:");
    
    let contents = fs::read_to_string("./inputs/input07.txt").expect("Not Found");


    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let map : Vec<Vec<char>> = contents.split('\n').map(|f| f.chars().collect()).collect();
    let y_len = map.len();
    let x_len = map[0].len();

    let start_pos = (map[0].iter().position(|f| f == &'S').unwrap(), 0);

    let mut split_pos: BTreeSet<(usize, usize)> = BTreeSet::new();
    track_down(start_pos, y_len, x_len, &map,&mut split_pos);




    println!("Part 1 Answer = {}", split_pos.len());
}

fn part2(contents: &str)
{
    let map : Vec<Vec<char>> = contents.split('\n').map(|f| f.chars().collect()).collect();
    let y_len = map.len();
    let x_len = map[0].len();

    let start_pos = (map[0].iter().position(|f| f == &'S').unwrap(), 0);

    let mut split_pos: HashMap<(usize, usize), i64> = HashMap::new();
    let timelines = track_down_timelines(start_pos, y_len, x_len, &map,&mut split_pos);

    println!("Part 2 Answer = {}", timelines);
}

fn track_down(curr_pos: (usize, usize), y_len: usize, x_len: usize, map: &Vec<Vec<char>>, split_pos_hit: &mut BTreeSet<(usize, usize)>) {
    if curr_pos.1 >= y_len || split_pos_hit.contains(&curr_pos){
        //println!("Return {} {}",  curr_pos.0, curr_pos.1);
        return;
    }

    //println!("Curr {} {}",  curr_pos.0, curr_pos.1);

    let next_char = map[curr_pos.1][curr_pos.0];
    if next_char != '^' {
        track_down((curr_pos.0, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Down {} {}",  curr_pos.0, curr_pos.1+1);
        return;
    }

    split_pos_hit.insert(curr_pos);
    if curr_pos.0 != 0 {
        track_down((curr_pos.0-1, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Left {} {}",  curr_pos.0-1, curr_pos.1+1);
    }
    if curr_pos.0 != x_len-1 { 
        track_down((curr_pos.0+1, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Right {} {}",  curr_pos.0+1, curr_pos.1+1);
    }

}

fn track_down_timelines (curr_pos: (usize, usize), y_len: usize, x_len: usize, map: &Vec<Vec<char>>, split_pos_hit: &mut HashMap<(usize, usize), i64>) -> i64{
    if curr_pos.1 >= y_len {
        //println!("Return {} {}",  curr_pos.0, curr_pos.1);
        return 1;
    }

    if split_pos_hit.contains_key(&curr_pos)
    {
        return *split_pos_hit.get(&curr_pos).unwrap();
    }

    //println!("Curr {} {}",  curr_pos.0, curr_pos.1);

    let mut timelines = 0;
    
    let next_char = map[curr_pos.1][curr_pos.0];
    if next_char != '^' {
        timelines += track_down_timelines((curr_pos.0, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Down {} {}",  curr_pos.0, curr_pos.1+1);
        return timelines;
    }

    if curr_pos.0 != 0 {
        timelines += track_down_timelines((curr_pos.0-1, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Left {} {}",  curr_pos.0-1, curr_pos.1+1);
    }
    if curr_pos.0 != x_len-1 { 
        timelines += track_down_timelines((curr_pos.0+1, curr_pos.1+1), y_len, x_len, map, split_pos_hit);
        //println!("Right {} {}",  curr_pos.0+1, curr_pos.1+1);
    }

    split_pos_hit.insert(curr_pos, timelines);

    return timelines;


}