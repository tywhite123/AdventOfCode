use std::fs;

fn main() {
    println!("Template:");
    
    let contents = fs::read_to_string("./inputs/input03.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let banks: Vec<Vec<u32>> = contents.split('\n').map(|f|{
        f.chars().enumerate().map(|(i,c)| c.to_digit(10).unwrap()).collect()
    }).collect();

    let mut total = 0;
    for bank in banks {
        let mut val1 = 0;
        let mut val2 = 0;
        let mut pos_of_val1 = 0;

        for val in 0..bank.len()-1 {
            if bank[val] > val1 {
                val1 = bank[val];
                pos_of_val1 = val;
            }
        }

        for valfor2 in pos_of_val1+1..bank.len() {
            if bank[valfor2] > val2 {
                val2 = bank[valfor2];
            }
        }

        total += (val1 * 10) + val2;

    }

    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let banks: Vec<Vec<u32>> = contents.split('\n').map(|f|{
        f.chars().enumerate().map(|(i,c)| c.to_digit(10).unwrap()).collect()
    }).collect();

    let mut total = 0;
    for bank in banks {
        let mut pos_of_last_val : i32 = -1;
        let mut combined = 0;
        for i in 1..=12 {
            let mut val = 0;
            for pos in (pos_of_last_val+1) as usize..(bank.len()-(12-i)){
                if bank[pos] > val {
                    val = bank[pos];
                    pos_of_last_val = pos as i32;
                }
            }

            combined = combined * 10 + val as u64;
        }
        
        total += combined;

    }

    println!("Part 2 Answer = {}", total);
}