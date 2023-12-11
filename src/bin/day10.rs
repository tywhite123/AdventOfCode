use std::fs;

fn main() {
    println!("Day 10:");
    
    let contents = fs::read_to_string("./inputs/input10.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let lines = contents.lines();
    let mut map_grid: Vec<Vec<char>> = Vec::new();

    let mut starting_pos = (0,0);
    for l in lines
    {
        let chars: Vec<char> = l.chars().collect();
        map_grid.push(chars);   
    }

    for y in 0..map_grid.len()
    {
        for x in 0..map_grid[y].len()
        {
            if map_grid[y][x] == 'S'
            {
                starting_pos = (x, y);
            }
        }
    }

    let mut fin = false;

    let mut current_pos = starting_pos;
    let mut last_pos = current_pos;
    let mut current_steps = 0;

    current_pos.0 += 1;

    while !fin
    {
        let temp_current = current_pos;
        match map_grid[current_pos.1][current_pos.0] {
            'S' => fin = true,
            '|' => if last_pos.1 < current_pos.1 {current_pos.1 += 1} else {current_pos.1 -= 1},
            '-' => if last_pos.0 < current_pos.0 {current_pos.0 += 1} else {current_pos.0 -= 1},
            'L' => if last_pos.1 < current_pos.1 {current_pos.0 += 1} else {current_pos.1 -= 1},
            'J' => if last_pos.1 < current_pos.1 {current_pos.0 -= 1} else {current_pos.1 -= 1},
            '7' => if last_pos.0 < current_pos.0 {current_pos.1 += 1} else {current_pos.0 -= 1},
            'F' => if last_pos.0 > current_pos.0 {current_pos.1 += 1} else {current_pos.0 += 1},
            _ => panic!("Panic")
        }

        last_pos = temp_current;
        current_steps += 1;
    }

    current_steps /= 2;

    println!("Part 1 Answer = {current_steps}");
}

fn part2(contents: &str)
{
    let lines = contents.lines();
    let mut map_grid: Vec<Vec<char>> = Vec::new();

    let mut starting_pos = (0,0);
    for l in lines
    {
        let chars: Vec<char> = l.chars().collect();
        map_grid.push(chars);   
    }

    for y in 0..map_grid.len()
    {
        for x in 0..map_grid[y].len()
        {
            if map_grid[y][x] == 'S'
            {
                starting_pos = (x, y);
            }
        }
    }
    

    let fin = false;

    let mut current_pos = starting_pos;
    let mut last_pos = current_pos;
    let mut visited_positions: Vec<(usize, usize)> = Vec::new();
    let mut verts: Vec<(usize, usize)> = Vec::new();
    visited_positions.push(starting_pos);
    verts.push(starting_pos);
    current_pos.0 += 1;

    while !fin
    {
        let temp_current = current_pos;
        match map_grid[current_pos.1][current_pos.0] {
            'S' => break,
            '|' => if last_pos.1 < current_pos.1 {current_pos.1 += 1} else {current_pos.1 -= 1},
            '-' => if last_pos.0 < current_pos.0 {current_pos.0 += 1} else {current_pos.0 -= 1},
            'L' => if last_pos.1 < current_pos.1 {current_pos.0 += 1} else {current_pos.1 -= 1},
            'J' => if last_pos.1 < current_pos.1 {current_pos.0 -= 1} else {current_pos.1 -= 1},
            '7' => if last_pos.0 < current_pos.0 {current_pos.1 += 1} else {current_pos.0 -= 1},
            'F' => if last_pos.0 > current_pos.0 {current_pos.1 += 1} else {current_pos.0 += 1},
            _ => panic!("Panic")
        }

        visited_positions.push(current_pos);

        if map_grid[current_pos.1][current_pos.0] == 'L' || map_grid[current_pos.1][current_pos.0] == 'J'
            || map_grid[current_pos.1][current_pos.0] == '7' || map_grid[current_pos.1][current_pos.0] == 'F'
        {
            verts.push(current_pos);
        }

        last_pos = temp_current;
    }

    let mut current_area = 0.0;

    for v in 0..verts.len()
    {
        let current = (verts[v].0 as f32, verts[v].1 as f32);
        let next = (verts[(v+1)%verts.len()].0 as f32, verts[(v+1)%verts.len()].1 as f32);

        current_area += (current.0*next.1) - (current.1*next.0);
    }

    current_area /= 2.0;
    let inside = current_area.abs() - (visited_positions.len() as f32) / 2.0 + 1.0;
    let inside_count = inside.floor();

    println!("Part 2 Answer = {inside_count}");
}