use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Day 06:");
    
    let contents = fs::read_to_string("./inputs/input06.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();
    let bounds = (split_values[0].len(), split_values.len()-1);
    let directions: [(isize, isize); 4] = [
        (0, -1), //NORTH
        (1, 0), //EAST
        (0, 1), //SOUTH
        (-1, 0) //WEST
    ];
    let mut current_dir_idx = 0;
    let mut current_guard_pos: (usize, usize) = (0, 0);

    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for y in 0..split_values.len()
    {
        let c: Vec<char> = split_values[y].chars().collect();
        for x in 0..c.len()
        {
            if c[x] == '^' {current_guard_pos = (x, y);}
            map.insert((x, y), c[x]);
        }
    }

    //println!("{:?}", map);

    let mut has_exit = false;
    let mut unique_travelled: HashSet<(usize, usize)> = HashSet::new();
    unique_travelled.insert(current_guard_pos);
    while !has_exit {
        let current_dir = directions[current_dir_idx];
        //println!("{:?}", current_guard_pos);
        let next_pos = (((current_guard_pos.0 as isize) + current_dir.0) as usize, ((current_guard_pos.1 as isize) + current_dir.1) as usize);
        if (current_guard_pos.0 as isize) + current_dir.0 == 0 || (current_guard_pos.0 as isize) + current_dir.0 == bounds.0 as isize
            || (current_guard_pos.1 as isize) + current_dir.1 == 0 || (current_guard_pos.1 as isize) + current_dir.1 == bounds.1 as isize
            {
                if *map.get(&next_pos).unwrap() == '#' { 
                    current_dir_idx = (current_dir_idx + 1) % directions.len();
                    continue;
                }
                has_exit = true
            }

        if *map.get(&next_pos).unwrap() == '#' { 
            current_dir_idx = (current_dir_idx + 1) % directions.len();
            continue;
        }
        
        current_guard_pos.0 = ((current_guard_pos.0 as isize) + current_dir.0) as usize;
        current_guard_pos.1 = ((current_guard_pos.1 as isize) + current_dir.1) as usize;
        unique_travelled.insert(current_guard_pos);
    }


    println!("Part 1 Answer = {}", unique_travelled.len());
}

fn part2(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();
    let bounds = (split_values[0].len(), split_values.len()-1);
    let directions: [(isize, isize); 4] = [
        (0, -1), //NORTH
        (1, 0), //EAST
        (0, 1), //SOUTH
        (-1, 0) //WEST
    ];
    let mut current_dir_idx = 0;
    let mut current_guard_pos: (usize, usize) = (0, 0);

    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for y in 0..split_values.len()
    {
        let c: Vec<char> = split_values[y].chars().collect();
        for x in 0..c.len()
        {
            if c[x] == '^' {current_guard_pos = (x, y);}
            map.insert((x, y), c[x]);
        }
    }

    //println!("{:?}", map);

    let s = map.clone();
    let mut loops = 0;
    let orig_guard_pos = current_guard_pos;
    for e in map.iter_mut()
    {
        if *e.1 != '.' {continue;}

        let mut route: HashSet<((usize, usize), usize)> = HashSet::new();

        let current_ob_pos: (usize, usize) = *e.0;

        let mut has_exit = false;
        while !has_exit {
            
            //if current_ob_pos == (3, 6) { println!("{:?}", current_guard_pos);} 
            let current_dir = directions[current_dir_idx];

            //println!("{:?}", current_guard_pos);
            let next_pos = (((current_guard_pos.0 as isize) + current_dir.0) as usize, ((current_guard_pos.1 as isize) + current_dir.1) as usize);
            if (current_guard_pos.0 as isize) + current_dir.0 == 0 || (current_guard_pos.0 as isize) + current_dir.0 == bounds.0 as isize
                || (current_guard_pos.1 as isize) + current_dir.1 == 0 || (current_guard_pos.1 as isize) + current_dir.1 == bounds.1 as isize
                {
                    //println!("{:?}", next_pos);
                    if let Some(&next_char) = s.get(&next_pos){
                        if next_char == '#' || next_pos == current_ob_pos {
                            //println!("HERE"); 
                            current_dir_idx = (current_dir_idx + 1) % directions.len();
                            continue;
                        }
                    }
                    has_exit = true;
                    break;
                }

            if *s.get(&next_pos).unwrap() == '#' || next_pos == current_ob_pos { 
                current_dir_idx = (current_dir_idx + 1) % directions.len();
                continue;
            } 
            
            

            if route.contains(&(current_guard_pos, current_dir_idx)) {
                //println!("{:?} {} : {:?} ", current_guard_pos, current_dir_idx, route);
                loops += 1;
                break;
            }
            route.insert((current_guard_pos, current_dir_idx));

            current_guard_pos.0 = ((current_guard_pos.0 as isize) + current_dir.0) as usize;
            current_guard_pos.1 = ((current_guard_pos.1 as isize) + current_dir.1) as usize;
        }

        current_guard_pos = orig_guard_pos;
        current_dir_idx = 0;
    }



    println!("Part 2 Answer = {}", loops);
}
