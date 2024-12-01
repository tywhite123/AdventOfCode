use std::fs;

fn main() {
    println!("Day 11:");
    
    let contents = fs::read_to_string("./inputs/input11.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let lines = contents.lines();
    let mut map_grid: Vec<Vec<char>> = Vec::new();

    let mut rows = 0;
    for line in lines
    {
        rows += 1;
        let chars: Vec<char> = line.chars().collect();
        map_grid.push(chars);   
    }
    let columns = map_grid.len();

    let mut expanded_map_grid: Vec<Vec<char>> = Vec::new();
    for y in 0..map_grid.len()
    {
        expanded_map_grid.push(map_grid[y].clone());
        if !map_grid[y].contains(&'#')
        {
            let row_copy = map_grid[y].clone();
            expanded_map_grid.push(row_copy);
        }
    }


    let mut current_offset = 1;
    for x in 0..rows
    {
        let mut expand_col = true;
        for y in 0..columns
        {
            if map_grid[y][x] == '#'
            {
                expand_col = false;
            }
        }

        if expand_col
        {
            for y in 0..expanded_map_grid.len()
            {
                //println!("{current_offset}");
                expanded_map_grid[y].insert(x+current_offset, '.');
            }
            current_offset+=1;
        }
    }    

    let mut galaxy_positions: Vec<(i32, i32)> = Vec::new();
    for y in 0..expanded_map_grid.len()
    {
        for x in 0..expanded_map_grid[y].len()
        {
            if expanded_map_grid[y][x] == '#'
            {
                let coord = (x as i32, y as i32);
                galaxy_positions.push(coord);
            }
        }
    }

    let mut total = 0;
    for i in 0..galaxy_positions.len()-1
    {
        for j in i+1..galaxy_positions.len()
        {
            let steps = (galaxy_positions[i].0 - galaxy_positions[j].0).abs() + (galaxy_positions[i].1 - galaxy_positions[j].1).abs();
            //println!("({} {}), ({} {}), {}", galaxy_positions[i].0, galaxy_positions[i].1, galaxy_positions[j].0, galaxy_positions[j].1, test);
            total+=steps;
        }
    }

    println!("Part 1 Answer = {total}");
}

fn part2(contents: &str)
{
    let lines = contents.lines();
    let mut map_grid: Vec<Vec<char>> = Vec::new();

    let mut rows = 0;
    for line in lines
    {
        rows += 1;
        let chars: Vec<char> = line.chars().collect();
        map_grid.push(chars);   
    }
    let columns = map_grid.len();

    let mut expanded_map_grid: Vec<Vec<char>> = Vec::new();
    for y in 0..map_grid.len()
    {
        expanded_map_grid.push(map_grid[y].clone());
        if !map_grid[y].contains(&'#')
        {
            let row_copy = map_grid[y].clone();
            expanded_map_grid.push(row_copy);
        }
    }


    let mut current_offset = 1;
    for x in 0..rows
    {
        let mut expand_col = true;
        for y in 0..columns
        {
            if map_grid[y][x] == '#'
            {
                expand_col = false;
            }
        }

        if expand_col
        {
            for y in 0..expanded_map_grid.len()
            {
                //println!("{current_offset}");
                expanded_map_grid[y].insert(x+current_offset, '.');
            }
            current_offset+=1;
        }
    }    

    let mut galaxy_positions: Vec<(i32, i32)> = Vec::new();
    for y in 0..map_grid.len()
    {
        for x in 0..map_grid[y].len()
        {
            if map_grid[y][x] == '#'
            {
                let coord = (x as i32, y as i32);
                galaxy_positions.push(coord);
            }
        }
    }

    let mut non_ex_total = 0;
    for i in 0..galaxy_positions.len()-1
    {
        for j in i+1..galaxy_positions.len()
        {
            let steps = ((galaxy_positions[i].0 - galaxy_positions[j].0).abs() + (galaxy_positions[i].1 - galaxy_positions[j].1).abs()) as i64;
            //println!("({} {}), ({} {}), {}", galaxy_positions[i].0, galaxy_positions[i].1, galaxy_positions[j].0, galaxy_positions[j].1, test);
            non_ex_total+=steps;
        }
    }

    let mut expanded_galaxy_positions: Vec<(i32, i32)> = Vec::new();
    for y in 0..expanded_map_grid.len()
    {
        for x in 0..expanded_map_grid[y].len()
        {
            if expanded_map_grid[y][x] == '#'
            {
                let coord = (x as i32, y as i32);
                expanded_galaxy_positions.push(coord);
            }
        }
    }

    let mut total = 0;
    for i in 0..expanded_galaxy_positions.len()-1
    {
        for j in i+1..expanded_galaxy_positions.len()
        {
            let steps = ((expanded_galaxy_positions[i].0 - expanded_galaxy_positions[j].0).abs() + (expanded_galaxy_positions[i].1 - expanded_galaxy_positions[j].1).abs()) as i64;
            //println!("({} {}), ({} {}), {}", galaxy_positions[i].0, galaxy_positions[i].1, galaxy_positions[j].0, galaxy_positions[j].1, test);
            total+=steps;
        }
    }

    let final_total = non_ex_total+(total-non_ex_total)*999999;

    println!("Part 2 Answer = {final_total}");
}