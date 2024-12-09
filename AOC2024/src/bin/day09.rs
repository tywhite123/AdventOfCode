use std::{collections::HashMap, fs, process::id};

fn main() {
    println!("Day 09:");
    
    let contents = fs::read_to_string("./inputs/input09.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) 
{
    //println!("{}", contents);
    let mut disk: Vec<String> = vec![];
    contents.chars().into_iter().enumerate().for_each(|(idx, char)| {
        let value_from_char = char.to_digit(10).unwrap();

        for _i in 0..value_from_char
        {
            if idx % 2 == 0 {
                let val= ((idx/2) as u32).to_string();
                disk.push(val);
            }
            else {
                disk.push((".").to_string());
            }
        } 
    });
    
    //println!("{:?}", disk);

    //let disk_clone = disk.clone();
    let mut sorted = false;
    for idx in (0..disk.len()).rev() {
        if disk[idx] == "." {continue;}

        for idx2 in 0..disk.len() {
            if idx < idx2 {
                sorted = true; 
                break;
            }

            if disk[idx2] == "."
            {
                disk.swap(idx, idx2);
                //println!("{:?}", disk);
                break;
            }
        } 

        if sorted {break;}    
    }

    let mut total: i64 = 0;
    for (i, c) in disk.iter().enumerate() {
        if *c == "." {break;}

        let value_from_char = c.parse::<i64>().unwrap();
        total += (i as i64) * value_from_char;

    }

    //println!("{:?}", disk);


    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    //println!("{}", contents);
    let mut disk: Vec<String> = vec![];
    let mut disk_map: HashMap<usize, i32> = HashMap::new();
    let mut new_idx = 0;
    contents.chars().into_iter().enumerate().for_each(|(idx, char)| {
        let value_from_char = char.to_digit(10).unwrap();

        disk_map.insert(new_idx, value_from_char as i32);
        new_idx = new_idx + (value_from_char as usize);

        for _i in 0..value_from_char
        {
            if idx % 2 == 0 {
                let val= ((idx/2) as u32).to_string();
                disk.push(val);
            }
            else {
                disk.push((".").to_string());
            }

        } 
    });
    
    //println!("{:?}", disk);

    //let disk_clone = disk.clone();

    let mut sorted = false;
    let mut idx = 0;
    let mut increment_by = 1;
    while idx < disk.len() {
        if disk[idx] != "." {
            idx += 1;
            continue;
        }

        if let Some(storage_block_size) = disk_map.get(&idx){
            let mut storage_left = *storage_block_size;
            increment_by = *storage_block_size;

            let mut current_check = (disk.len() as i32)-1;
            //println!("{} {} {}", idx, current_check, disk.len());
            let mut pos_to_fill = idx as i32;
            while storage_left > 0 {
                if current_check < idx as i32 {
                    //sorted = true;
                    break;
                }
 
                if disk[current_check as usize] == "." {
                    current_check -= 1;
                    // println!("{} {} {}", idx, current_check, disk.len());
                    continue;
                }

                if let Some(size) = disk_map.get(&(current_check as usize)){
                    if *size <= storage_left {
                        //println!("{} {}", disk[current_check as usize], disk[pos_to_fill as usize]);
                        for i in 0..*size {
                            disk.swap((current_check + i) as usize, pos_to_fill as usize);
                            pos_to_fill += 1;
                            storage_left -= 1;
                        }
                        //println!("{:?}", disk);
                    } 
                }

                current_check-=1;

            }
        }
        
        if sorted {break;}
        idx += increment_by as usize;

    }


    let mut total: i64 = 0;
    for (i, c) in disk.iter().enumerate() {
        if *c == "." {continue;}

        let value_from_char = c.parse::<i64>().unwrap();
        total += (i as i64) * value_from_char;
        //println!("{} * {} = {}", i, value_from_char, total);

    }

    //println!("{:?}", disk);


    println!("Part 2 Answer = {}", total);
}