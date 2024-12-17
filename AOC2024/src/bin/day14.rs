use std::{fs::{self, File}, io::Write};

use itertools::Itertools;

fn main() {
    println!("Template:");
    
    let contents = fs::read_to_string("./inputs/input14.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str){
    let mut lines: Vec<(i32, i32)> = contents.lines().flat_map(|f| {
        f.split(" ").into_iter().map(|part| {
            let (mut x, mut y): (i32, i32) = (0, 0);
            if part.starts_with("p=") {
                let (x_s, y_s) = part.trim_start_matches("p=").split(",").collect_tuple().unwrap();
                x = x_s.parse::<i32>().unwrap();
                y = y_s.parse::<i32>().unwrap();
            }
            else {
                let (x_s, y_s) = part.trim_start_matches("v=").split(",").collect_tuple().unwrap();
                x = x_s.parse::<i32>().unwrap();
                y = y_s.parse::<i32>().unwrap();
            }

            (x, y)
        })
    }).collect();


    let (grid_size_x, grid_size_y) = (101, 103);

    for _second in 0..100 {
        for i in (0..lines.len()).step_by(2) {
            let (p_x, p_y) = lines[i];
            let (v_x, v_y) = lines[i+1];

            let (new_x, new_y) = (((p_x + v_x) + grid_size_x) % grid_size_x, ((p_y + v_y) + grid_size_y) % grid_size_y);

            lines[i].0 = new_x;
            lines[i].1 = new_y;
        }
    }

    let mut quadrants: Vec<i32> = vec![0; 4];


    for i in (0..lines.len()).step_by(2) {
        let (x, y) = lines[i];

        if x >= 0 && x < grid_size_x/2 && y >= 0 && y < grid_size_y/2 {
            quadrants[0] += 1;
        }       
        else if x >= grid_size_x/2+1 && x < grid_size_x && y >= 0 && y < grid_size_y/2 {
            quadrants[1] += 1;
        }       
        else if x >= 0 && x < grid_size_x/2 && y >= grid_size_y/2+1 && y < grid_size_y {
            quadrants[2] += 1;
        }       
        else if x >= grid_size_x/2+1 && x < grid_size_x && y >= grid_size_y/2+1 && y < grid_size_y  {
            quadrants[3] += 1;
        }       
    }

    let total = quadrants.iter().fold(1, |acc, val| acc * val); 

    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str) -> std::io::Result<()>
{
    let mut lines: Vec<(i32, i32)> = contents.lines().flat_map(|f| {
        f.split(" ").into_iter().map(|part| {
            let (mut x, mut y): (i32, i32) = (0, 0);
            if part.starts_with("p=") {
                let (x_s, y_s) = part.trim_start_matches("p=").split(",").collect_tuple().unwrap();
                x = x_s.parse::<i32>().unwrap();
                y = y_s.parse::<i32>().unwrap();
            }
            else {
                let (x_s, y_s) = part.trim_start_matches("v=").split(",").collect_tuple().unwrap();
                x = x_s.parse::<i32>().unwrap();
                y = y_s.parse::<i32>().unwrap();
            }

            (x, y)
        })
    }).collect();


    let (grid_size_x, grid_size_y) = (101, 103);

    let mut file = File::create("output_day14.txt")?;

    for second in 0..10000 {
        let mut current_positions: Vec<(i32, i32)> = vec![];
        for i in (0..lines.len()).step_by(2) {
            let (p_x, p_y) = lines[i];
            let (v_x, v_y) = lines[i+1];

            let (new_x, new_y) = (((p_x + v_x) + grid_size_x) % grid_size_x, ((p_y + v_y) + grid_size_y) % grid_size_y);

            lines[i].0 = new_x;
            lines[i].1 = new_y;

            current_positions.push((new_x, new_y));
        }
        

        writeln!(&mut file, "{}", second);
        for y in 0..grid_size_y {
            for x in 0..grid_size_x {
                if current_positions.contains(&(x, y)) {
                    //current_positions.
                    write!(&mut file, "R");
                }
                else {
                    write!(&mut file, ".");
                }

            }
            writeln!(&mut file, "");
        }
        writeln!(&mut file, "");
    }

    let mut quadrants: Vec<i32> = vec![0; 4];


    for i in (0..lines.len()).step_by(2) {
        let (x, y) = lines[i];

        if x >= 0 && x < grid_size_x/2 && y >= 0 && y < grid_size_y/2 {
            quadrants[0] += 1;
        }       
        else if x >= grid_size_x/2+1 && x < grid_size_x && y >= 0 && y < grid_size_y/2 {
            quadrants[1] += 1;
        }       
        else if x >= 0 && x < grid_size_x/2 && y >= grid_size_y/2+1 && y < grid_size_y {
            quadrants[2] += 1;
        }       
        else if x >= grid_size_x/2+1 && x < grid_size_x && y >= grid_size_y/2+1 && y < grid_size_y  {
            quadrants[3] += 1;
        }       
    }

    let total = quadrants.iter().fold(1, |acc, val| acc * val); 

    println!("Part 2 Answer = {}", total);
    Ok(())
}