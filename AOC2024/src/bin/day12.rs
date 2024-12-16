use std::{collections::{HashMap, HashSet}, fs};


#[derive(Clone)]
struct Plot {
    plant: char,
    position: (i32,i32)
}

struct Region {
    plant: char,
    plots: Vec<Plot>,
    area: i32,
    perimeter: i32
}

fn main() {
    println!("Day 12:");
    
    let contents = fs::read_to_string("./inputs/input12.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let max_y = contents.lines().count() as i32;
    let max_x = contents.lines().nth(0).unwrap().len() as i32;

    let plots: HashMap<(i32,i32), Plot> = contents
        .lines()
        .enumerate()
        .flat_map(|(y, f)| f.chars()
            .enumerate()
            .map(move |(x, f)| 
                ((x as i32, y as i32), Plot{plant: f, position: (x as i32, y as i32)})
            )).collect();

    let regions: Vec<Region> = find_regions(plots, (max_x, max_y));

    let mut total_price = 0;
    for mut region in regions {
        region.area = region.plots.len() as i32;

        region.perimeter = calculate_perimeter(&mut region);

        total_price += region.area * region.perimeter;
    }



    println!("Part 1 Answer = {}", total_price);
}

fn part2(contents: &str)
{
    let max_y = contents.lines().count() as i32;
    let max_x = contents.lines().nth(0).unwrap().len() as i32;

    let plots: HashMap<(i32,i32), Plot> = contents
        .lines()
        .enumerate()
        .flat_map(|(y, f)| f.chars()
            .enumerate()
            .map(move |(x, f)| 
                ((x as i32, y as i32), Plot{plant: f, position: (x as i32, y as i32)})
            )).collect();

    let regions: Vec<Region> = find_regions(plots, (max_x, max_y));

    let mut total_price = 0;
    for mut region in regions {
        region.area = region.plots.len() as i32;

        region.perimeter = calculate_num_sides(&mut region);

        total_price += region.area * region.perimeter;
    }

    println!("Part 2 Answer = {}", total_price);
}

fn find_regions(plots: HashMap<(i32,i32), Plot>, (max_x, max_y): (i32, i32)) -> Vec<Region> {
    let mut regions_found: Vec<Region> = vec![];
    let mut positions_handled: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..max_y {
        for x in 0..max_x {
            if positions_handled.contains(&(x, y)) { continue; }

            let mut current_region: Region = Region{plant: plots.get(&(x,y)).unwrap().plant, plots: vec![], area: 0, perimeter: 0};
            find_and_add_plots((x,y), &plots, &mut current_region, &mut positions_handled, (max_x, max_y)); 
            regions_found.push(current_region);
        }
    }


    return regions_found;
}

fn find_and_add_plots(current_plot: (i32, i32), plots: &HashMap<(i32,i32), Plot>, region: &mut Region, positions_handled: &mut HashSet<(i32, i32)>, (max_x, max_y): (i32, i32)){
    if current_plot.0 < 0 || current_plot.1 < 0 || current_plot.0 >= max_x || current_plot.1 >= max_y {return;}

    let current = &plots[&current_plot];
    if positions_handled.contains(&current.position) { return; }

    if current.plant != region.plant { return; }

    positions_handled.insert(current.position);
    region.plots.push(Plot{plant: current.plant, position: current.position});

    let directions = [(-1,0), (1, 0), (0, -1), (0 , 1)];
    for dir in directions {
        find_and_add_plots((current_plot.0 + dir.0, current_plot.1 + dir.1), plots, region, positions_handled, (max_x, max_y));
    }
}

fn calculate_perimeter(region: &mut Region) -> i32 {
    let points: HashSet<(i32,i32)> = region.plots.iter().map(|f| f.position).collect();
    let mut perimeter: Vec<(i32, i32)> = vec![];

    let mut perimeter_len = 0;

    for plot in region.plots.clone().into_iter() {
        let neighbors = [
            (plot.position.0 - 1, plot.position.1), // Left
            (plot.position.0 + 1, plot.position.1), // Right
            (plot.position.0, plot.position.1 - 1), // Up
            (plot.position.0, plot.position.1 + 1), // Down
        ];

        if neighbors.iter().any(|&n| !points.contains(&n)) {
            perimeter.push((plot.position.0, plot.position.1));

            perimeter_len = neighbors.iter().fold(perimeter_len, |sum, n| {
                let mut sides = 0;
                if !points.contains(n) {
                    sides += 1;
                }

                sum + sides
            })
        }
    }

    return perimeter_len;
}

fn calculate_num_sides(region: &mut Region) -> i32 {
    let points: HashSet<(i32,i32)> = region.plots.iter().map(|f| f.position).collect();
    let mut perimeter: Vec<(i32, i32)> = vec![];

    let mut corners = 0;

    for plot in region.plots.clone().into_iter() {
        let neighbors = [
            ((plot.position.0 - 1, plot.position.1), 1), // Left
            ((plot.position.0 + 1, plot.position.1), 2), // Right
            ((plot.position.0, plot.position.1 - 1), 4), // Up
            ((plot.position.0, plot.position.1 + 1), 8), // Down
        ];

        let diagonal_neighbors = [
            ((plot.position.0 - 1, plot.position.1 - 1), 1, (2, 0)), // Top Left
            ((plot.position.0 + 1, plot.position.1 - 1), 2, (2, 1)), // Top Right
            ((plot.position.0 - 1, plot.position.1 + 1), 4, (3, 0)), // Bottom Left
            ((plot.position.0 + 1, plot.position.1 + 1), 8, (3, 1)), // Bottom Right
        ];

        let mut contains: HashSet<(i32, i32)> = HashSet::new();

        let mut bitmask_adjacency: u8 = 0;
        for n in neighbors {
            if !points.contains(&n.0) {
                bitmask_adjacency |= n.1;
            }
            else {
                contains.insert(n.0);
            } 
        }

        match bitmask_adjacency {
            0 | 1 | 2 | 4 | 8  => corners += 0,
            3 | 12 => corners += 0,
            5 | 6 | 9 | 10 => corners += 1,
            7 | 11 | 13 | 14 => corners += 2,
            15 => corners += 4,
            16.. => corners += 0
        }

        let mut bitmask_opposite: u8 = 0;
        for n in diagonal_neighbors {
            if !points.contains(&n.0){
                if points.contains(&neighbors[n.2.0].0) && points.contains(&neighbors[n.2.1].0)
                {
                    bitmask_opposite |= n.1;
                }
            }
        } 

        //println!("BITMASK FOR {:?}: {}", plot.position, bitmask_opposite);
        match bitmask_opposite {
            0  => corners += 0,
            1 | 2 | 4 | 8  => corners += 1,
            3 | 12 => corners += 2,
            5 | 6 | 9 | 10 => corners += 2,
            7 | 11 | 13 | 14 => corners += 3,
            15 => corners += 4,
            16.. => corners += 0 
        }




    }

    //let ordered = order_perimeter_points(perimeter);
    //let mut last_point = ordered[0];
    //let mut direction = (1,0);

    //for i in 1..ordered.len() {
    //    let point = ordered[i];
    //    let current_dir = (point.0 - last_point.0, point.1 - last_point.1);

    //    if current_dir != direction {
    //        sides += 1;
    //    }

    //    direction = current_dir;
    //    last_point = point;
    //}

    //sides += 3;

    //println!("Plant: {}, Points: {:?}", region.plant, ordered);


    println!("{} region has {} sides", region.plant, corners);

    return corners;
}

fn order_perimeter_points(mut points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // Find the top-left point as the starting point
    points.sort_by_key(|&(x, y)| (y, x));
    let start = points[0];

    // Sort the remaining points by angle relative to the starting point
    points.sort_by(|&(x1, y1), &(x2, y2)| {
        let dx1 = x1 - start.0;
        let dy1 = y1 - start.1;
        let dx2 = x2 - start.0;
        let dy2 = y2 - start.1;

        (dy1 as f64).atan2(dx1 as f64)
            .partial_cmp(&(dy2 as f64).atan2(dx2 as f64))
            .unwrap()
    });

    points
}
