use std::fs;
use cached::proc_macro::cached;

fn main() {
    println!("Day 11:");
    
    let contents = fs::read_to_string("./inputs/input11.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let values: Vec<i64> = contents.split(' ').map(|f| f.parse::<i64>().unwrap()).collect();

    //for _i in 0..25 as i64 {
    //    blink(&mut values);
    //    //println!("{:?}", values);
    //}
    let mut total = 0;
    for stone in values {
        total += count_stones(stone, 25);
    }
    
    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let values: Vec<i64> = contents.split(' ').map(|f| f.parse::<i64>().unwrap()).collect();

    //for i in 0..75 as i64 {
    //    blink(&mut values);
    //    println!("{}", i);
    //}

    let mut total = 0;
    for stone in values {
        total += count_stones(stone, 75);
    }

    println!("Part 2 Answer = {}", total);
}

fn blink(stones: &mut Vec<i64>){
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i].to_string();

        if stones[i] == 0 {
            stones[i] = 1;
        }
        else if stone.len() % 2 == 0 {
            let (n1, n2) = stone.split_at(stone.len()/2);
            stones.remove(i);
            stones.insert(i, n1.parse::<i64>().unwrap());
            i+=1;
            stones.insert(i, n2.parse::<i64>().unwrap());

        }
        else {
            stones[i] *= 2024;
        }
        i+=1;
        
    }

}

#[cached]
fn blink_v2(stone: i64) -> (i64, Option<i64>){

    if stone == 0{
        return (1, None);
    }

    let stone_string = stone.to_string();
    if stone_string.len() % 2 == 0 {
        let (n1, n2) = stone_string.split_at(stone_string.len()/2);
        
        
        let left =  n1.parse::<i64>().unwrap();
        let right =  n2.parse::<i64>().unwrap();

        return (left, Some(right));

    }

    return (stone*2024, None);
}

#[cached]
fn count_stones(stone: i64, depth: i64) -> i64 {

    let (left, right) = blink_v2(stone);

    if depth == 1{
        if right == None {
            return 1;
        }
        else {
            return 2;
        }
    }

    let mut out = count_stones(left, depth-1);
    if right != None {
        out += count_stones(right.unwrap(), depth-1);
    }

    return out;
}