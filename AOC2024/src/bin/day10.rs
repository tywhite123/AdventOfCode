use std::{collections::{HashSet, VecDeque}, fs};

fn main() {
    println!("Day 10:");
    
    let contents = fs::read_to_string("./inputs/input10.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let topographic_map = parse_map(&contents);
    let total_score = calculate_total_score(topographic_map);
        
    println!("Part 1 Answer = {}", total_score);
}

fn part2(contents: &str)
{
    let topographic_map = parse_map(&contents);
    let total_score = calculate_total_score2(topographic_map);
    println!("Part 2 Answer = {}", total_score);
}


fn parse_map(input_map: &str) -> Vec<Vec<i32>> {
    input_map
        .trim()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn find_trailheads(topographic_map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (r, row) in topographic_map.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((r, c));
            }
        }
    }
    trailheads
}

fn bfs(topographic_map: &Vec<Vec<i32>>, start: (usize, usize)) -> i32 {
    let (rows, cols) = (topographic_map.len(), topographic_map[0].len());
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((r, c)) = queue.pop_front() {
        let current_height = topographic_map[r][c];

        for &(dr, dc) in &directions {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < rows && nc < cols && !visited.contains(&(nr, nc)) {
                let next_height = topographic_map[nr][nc];

                if next_height == current_height + 1 { // Valid uphill step
                    visited.insert((nr, nc));
                    queue.push_back((nr, nc));

                    if next_height == 9 {
                        reachable_nines.insert((nr, nc));
                    }
                }
            }
        }
    }

    reachable_nines.len() as i32
}

fn calculate_total_score(topographic_map: Vec<Vec<i32>>) -> i32 {
    let trailheads = find_trailheads(&topographic_map);
    let mut total_score = 0;

    for trailhead in trailheads {
        total_score += bfs(&topographic_map, trailhead);
    }

    total_score
}

fn bfs2(topographic_map: &Vec<Vec<i32>>, start: (usize, usize)) -> i32 {
    let (rows, cols) = (topographic_map.len(), topographic_map[0].len());
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distinct_paths_to_nine = HashSet::new();

    queue.push_back((start, vec![start]));
    visited.insert(start);

    while let Some((current_pos, path)) = queue.pop_front() {
        let (r, c) = current_pos;
        let current_height = topographic_map[r][c];

        for &(dr, dc) in &directions {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < rows && nc < cols {
                let next_height = topographic_map[nr][nc];
                let next_pos = (nr, nc);

                // Traverse only if valid and not already part of the current path
                if next_height == current_height + 1 && !path.contains(&next_pos) {
                    let mut new_path = path.clone();
                    new_path.push(next_pos);

                    if next_height == 9 {
                        distinct_paths_to_nine.insert(new_path);
                    } else {
                        queue.push_back((next_pos, new_path));
                    }
                }
            }
        }
    }

    distinct_paths_to_nine.len() as i32
}

fn calculate_total_score2(topographic_map: Vec<Vec<i32>>) -> i32 {
    let trailheads = find_trailheads(&topographic_map);
    let mut total_score = 0;

    for trailhead in trailheads {
        total_score += bfs2(&topographic_map, trailhead);
    }

    total_score
}