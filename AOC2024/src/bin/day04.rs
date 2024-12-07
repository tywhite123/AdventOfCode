use std::fs;

fn main() {
    println!("Day 04:");
    
    let contents = fs::read_to_string("./inputs/input04.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
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

    let search_word = "XMAS";

    let chars: Vec<char> = split_values.clone().into_iter().flat_map(|s| s.chars()).collect();
    

    //let mut current_char_to_find: usize = 0;
    let mut total_found = 0;


    for y in 0..columns
    {
        for x in 0..line_len
        {
            let mut current_char_to_find: usize = 0;
            let letter = chars[x+(y*line_len)];

            if letter == search_word.chars().nth(current_char_to_find).unwrap()
            {

                for d in directions
                {
                    let mut found = false;
                    let mut x32 = x as i32;
                    let mut y32 = y as i32;
                    current_char_to_find = 1;

                    while !found
                    {
                        if x32+d.0 < 0 || x32+d.0 >= (line_len as i32) 
                            || y32+d.1 < 0 || y32+d.1 >= (columns as i32) {break;}

                        let idx = ((x32+d.0)+((y32+d.1)*(line_len as i32))) as usize;
                        if chars[idx] != search_word.chars().nth(current_char_to_find).unwrap() { break; }

                        if chars[idx] == search_word.chars().nth(current_char_to_find).unwrap() 
                            && chars[idx] == 'S' {
                            found = true;
                            total_found += 1;
                            
                        }
                        
                        current_char_to_find+=1;
                        x32 += d.0;
                        y32 += d.1;
                    }
                }
            }
        }
    }

    println!("Part 1 Answer = {}", total_found);
}

fn part2(contents: &str)
{
    let directions = [
       ((-1,-1),(1,1)),
       ((-1,1), (1,-1)) 
    ];
    let split_values : Vec<&str> = contents.split('\n').collect();
    let line_len = split_values[0].len();
    let columns = contents.len()/line_len;


    let chars: Vec<char> = split_values.clone().into_iter().flat_map(|s| s.chars()).collect();
    

    let mut total_found = 0;


    for y in 0..columns
    {
        for x in 0..line_len
        {
            let letter = chars[x+(y*line_len)];

            if letter == 'A' 
            {
                let mut found = true;
                for d in directions
                {
                    let x32 = x as i32;
                    let y32 = y as i32;
                    
                    if x32+d.0.0 < 0 || x32+d.0.0 >= (line_len as i32) 
                        || y32+d.0.1 < 0 || y32+d.0.1 >= (columns as i32) {found = false; break;}
                        
                    if x32+d.1.0 < 0 || x32+d.1.0 >= (line_len as i32) 
                        || y32+d.1.1 < 0 || y32+d.1.1 >= (columns as i32) {found = false; break;}

                    let idx_a = ((x32+d.0.0)+((y32+d.0.1)*(line_len as i32))) as usize;
                    let idx_b = ((x32+d.1.0)+((y32+d.1.1)*(line_len as i32))) as usize;
                    if (chars[idx_a] == 'M' && chars[idx_b] == 'S') 
                        || (chars[idx_a] == 'S' && chars[idx_b] == 'M') {
                        found = true;
                        continue;
                    }
                    
                    found = false;
                    break; 
                }

                if found {
                    total_found += 1;
                }
            }
        }
    }

    println!("Part 2 Answer = {}", total_found);
}