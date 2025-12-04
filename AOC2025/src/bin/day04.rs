use std::{fs, str::RMatchIndices};

fn main() {
    println!("Day 04:");
    
    let contents = fs::read_to_string("./inputs/input04.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    const CHECK_CHAR: char = '@';


    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
        (1, 1),  // diagonal down-right
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
        (-1, 1), // diagonal up-right
    ];
    let split_values : Vec<&str> = contents.split('\n').collect();
    let line_len = split_values[0].len();
    let columns = contents.len()/line_len;


    let chars: Vec<char> = split_values.clone().into_iter().flat_map(|s| s.chars()).collect();

    let mut total_accessible = 0;

    for y in 0..columns
    {
        for x in 0..line_len
        {
            let mut total_char_found = 0;
            let letter = chars[x+(y*line_len)];

            if letter == CHECK_CHAR
            {
                let mut x32 = x as i32;
                let mut y32 = y as i32;
                for d in directions
                {
                    if total_char_found >= 4 {break;}

                    if x32+d.0 < 0 || x32+d.0 >= (line_len as i32) 
                        || y32+d.1 < 0 || y32+d.1 >= (columns as i32) {continue;}

                    let idx = ((x32+d.0)+((y32+d.1)*(line_len as i32))) as usize;
                    if chars[idx] == CHECK_CHAR {
                        total_char_found += 1;
                    }
                }

                if total_char_found < 4 {
                    total_accessible += 1;
                }
            }
        }
    }
    println!("Part 1 Answer = {}", total_accessible);
}

fn part2(contents: &str)
{
    const CHECK_CHAR: char = '@';

    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
        (1, 1),  // diagonal down-right
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
        (-1, 1), // diagonal up-right
    ];
    let split_values : Vec<&str> = contents.split('\n').collect();
    let line_len = split_values[0].len();
    let columns = contents.len()/line_len;


    let mut chars: Vec<char> = split_values.clone().into_iter().flat_map(|s| s.chars()).collect();

    let mut total_accessible = 0;
    let mut non_removed = false;
    let mut round = 0;
    while !non_removed {
        let mut pos_to_remove: Vec<usize> = vec![];
        non_removed = true;

        let mut removed_this_round = 0;
        for y in 0..columns
        {
            for x in 0..line_len
            {
                let mut total_char_found = 0;
                let letter = chars[x+(y*line_len)];

                if letter == CHECK_CHAR
                {
                    let mut x32 = x as i32;
                    let mut y32 = y as i32;
                    for d in directions
                    {
                        if total_char_found >= 4 {break;}

                        if x32+d.0 < 0 || x32+d.0 >= (line_len as i32) 
                            || y32+d.1 < 0 || y32+d.1 >= (columns as i32) {continue;}
                    
                        let idx = ((x32+d.0)+((y32+d.1)*(line_len as i32))) as usize;
                        

                        if chars[idx] == CHECK_CHAR {
                            total_char_found += 1;
                        }
                    }

                    if total_char_found < 4 {
                        pos_to_remove.push(x+(y*line_len));
                        total_accessible += 1;
                        non_removed = false;
                        removed_this_round += 1;
                    }
                }
            }
        }

        for pos in pos_to_remove {
            chars[pos] = '.';
        }

        if !non_removed
        {
            println!("Round {}: {}", round, removed_this_round);
        }
        round += 1;
    }

    for y in 0..columns
    {
        for x in 0..line_len
        {
            let letter = chars[x+(y*line_len)];
            print!("{}", letter);
        }
        println!("");
    }

    println!("Part 2 Answer = {}", total_accessible);
}