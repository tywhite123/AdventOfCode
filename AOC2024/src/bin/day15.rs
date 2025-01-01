use std::fs;

use itertools::Itertools;

fn main() {
    println!("Day 15:");
    
    let contents = fs::read_to_string("./inputs/test15.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let split: Vec<&str> = contents.split("\n\n").collect();

    let y_size = split[0].lines().count() as i32;
    let x_size = split[0].lines().nth(0).unwrap().len() as i32;
    
    let mut grid: Vec<char> = split[0].lines().flat_map(|f| f.chars()).collect();

    let start = grid.iter().find_position(|&ch| *ch == '@').unwrap().0 as i32;
    //println!("{}", s);

    //let start = split[0].find("@").unwrap() as i32;
    let y = start/y_size;
    let (start_x, start_y) = (start-(y_size*y), y); 

    let moves: Vec<char> = split[1].chars().collect();

    let (mut current_x, mut current_y) = (start_x, start_y); 
    for m in moves {
        match m {
            '<' => (current_x, current_y) = move_dir((-1, 0), (current_x, current_y), &mut grid, (x_size, y_size)),
            '^' => (current_x, current_y) = move_dir((0, -1), (current_x, current_y), &mut grid, (x_size, y_size)),
            '>' => (current_x, current_y) = move_dir((1, 0), (current_x, current_y), &mut grid, (x_size, y_size)),
            'v' => (current_x, current_y) = move_dir((0, 1), (current_x, current_y), &mut grid, (x_size, y_size)),
            _ => (current_x, current_y) = (current_x, current_y)
        }

    }

    let sum = grid.iter().enumerate().fold(0, | acc, (idx, x)| {
        let mut to_add = 0;
        if *x == 'O' {
            let y = idx as i32/x_size;
            let (p_x, p_y) = (idx as i32-(x_size*y), y); 

            to_add += 100 * p_y + p_x;
        }
        acc + to_add
    });

    println!("Part 1 Answer = {}", sum);
}

#[derive(PartialEq)]
enum Action {
    INSERT,
    REPLACE
}

fn part2(contents: &str)
{
    let split: Vec<&str> = contents.split("\n\n").collect();

    let y_size = split[0].lines().count() as i32;
    let x_size = split[0].lines().nth(0).unwrap().len() as i32*2;
    
    let mut grid: Vec<char> = split[0].lines().flat_map(|f| f.chars()).collect();

    let mut current_offset = 0;
    let mut grid_updates: Vec<(usize, char, Action)> = vec![];
    grid.iter().enumerate().for_each(|(idx, c)|{
        match *c {
            '#' => {
                grid_updates.push((idx+current_offset+1, *c, Action::INSERT));
                current_offset += 1;
            },
            'O' => {
                grid_updates.push((idx+current_offset, '[', Action::REPLACE));
                grid_updates.push((idx+current_offset+1, ']', Action::INSERT));
                current_offset += 1;
            },
            '@' | '.' =>  {
                grid_updates.push((idx+current_offset+1, '.', Action::INSERT));
                current_offset += 1;
            },
            _ => todo!()
        }
    
    });

    for g in grid_updates {
        if g.2 == Action::INSERT {
            grid.insert(g.0, g.1);
        }
        else if g.2 == Action::REPLACE {
            grid[g.0] = g.1;
        }
    }

    for y in 0..y_size {
        for x in 0..x_size {
            let c = grid[(x + (y*(x_size))) as usize];
            print!("{}", c);
        }
        println!("");
    }

    let start = grid.iter().find_position(|&ch| *ch == '@').unwrap().0 as i32;
    ////println!("{}", s);

    //let start = split[0].find("@").unwrap() as i32;
    let y = start/x_size;
    let (start_x, start_y) = (start-(x_size*y), y); 

    let moves: Vec<char> = split[1].chars().collect();

    let (mut current_x, mut current_y) = (start_x, start_y); 
    println!("{} {}", current_x, current_y);
    for m in moves {
        match m {
            '<' => (current_x, current_y) = move_dir2((-1, 0), (current_x, current_y), &mut grid, (x_size, y_size)),
            '^' => (current_x, current_y) = move_dir2((0, -1), (current_x, current_y), &mut grid, (x_size, y_size)),
            '>' => (current_x, current_y) = move_dir2((1, 0), (current_x, current_y), &mut grid, (x_size, y_size)),
            'v' => (current_x, current_y) = move_dir2((0, 1), (current_x, current_y), &mut grid, (x_size, y_size)),
            _ => (current_x, current_y) = (current_x, current_y)
        }

        println!("{}", m);
        for y in 0..y_size {
            for x in 0..x_size {
                let c = grid[(x + (y*(x_size))) as usize];
                print!("{}", c);
            }
            println!("");
        }
        println!("{} {}", current_x, current_y);
    }

    let sum = grid.iter().enumerate().fold(0, | acc, (idx, x)| {
        let mut to_add = 0;
        if *x == '[' {
            let y = idx as i32/x_size;
            let (p_x, p_y) = (idx as i32-(x_size*y), y); 

            to_add += 100 * p_y + p_x;
        }
        acc + to_add
    });


    println!("Part 2 Answer = {}", sum);
}

fn move_dir(dir: (i32, i32), (x, y): (i32, i32), grid: &mut Vec<char>, (x_max, y_max): (i32, i32)) -> (i32, i32) {
    let pos_to_check = (x+dir.0, y+dir.1);

    let char = grid[(pos_to_check.0 + (pos_to_check.1 * x_max)) as usize];
    if char == '#' {
        return (x, y);
    }
    else if char == 'O' {
        let move_pos = move_dir(dir, pos_to_check, grid, (x_max, y_max));
        if move_pos == pos_to_check {
            //println!("HERE");
            return (x, y);
        }
    }

    //println!("HERE 2");
    grid[(pos_to_check.0 + (pos_to_check.1 * x_max)) as usize] = grid[(x + (y * x_max)) as usize];
    grid[(x + (y * x_max)) as usize] = '.';
    return pos_to_check; 
}

fn move_dir2(dir: (i32, i32), (x, y): (i32, i32), grid: &mut Vec<char>, (x_max, y_max): (i32, i32)) -> (i32, i32) {
    let pos_to_check = (x+dir.0, y+dir.1);

    let char = grid[(pos_to_check.0 + (pos_to_check.1 * x_max)) as usize];
    let char2 = grid[((pos_to_check.0+1) + (pos_to_check.1 * x_max)) as usize];
    let char3 = grid[((pos_to_check.0-1) + (pos_to_check.1 * x_max)) as usize];
    if char == '#' || char2 == '#'{
        return (x, y);
    }
    else if char == '[' && char2 == ']'{
        let move_pos = move_dir2(dir, pos_to_check, grid, (x_max, y_max));
        move_dir2(dir, (pos_to_check.0+1, pos_to_check.1), grid, (x_max, y_max)); 
        //let move_pos = move_dir2(dir, pos_to_check, grid, (x_max, y_max));
        if move_pos == pos_to_check {
            //println!("HERE");
            return (x, y);
        }
    }
    else if char == ']'  && char3 == '['{
        let move_pos = move_dir2(dir, pos_to_check, grid, (x_max, y_max));
        move_dir2(dir, (pos_to_check.0-1, pos_to_check.1), grid, (x_max, y_max)); 
        if move_pos == pos_to_check {
            //println!("HERE");
            return (x, y);
        }
    }

    //println!("HERE 2");
    grid[(pos_to_check.0 + (pos_to_check.1 * x_max)) as usize] = grid[(x + (y * x_max)) as usize];
    grid[(x + (y * x_max)) as usize] = '.';
    return pos_to_check; 
}