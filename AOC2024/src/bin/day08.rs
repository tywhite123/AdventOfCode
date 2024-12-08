use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    println!("Day 08:");
    
    let contents = fs::read_to_string("./inputs/input08.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let split_lines: Vec<&str> = contents.split('\n').collect();
    let map: HashMap<(i32, i32), char> = split_lines.iter()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars()
        .enumerate() // Get index (column number)
        .map(move |(x, c)| ((x as i32, y as i32), c))
    }).collect();

    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..split_lines.len() as i32{
        for x in 0..split_lines[0].len() as i32{
            let current_char = map.get(&(x,y)).unwrap();
            if *current_char == '.' {continue;}

            for y2 in 0..split_lines.len() as i32{
                for x2 in 0..split_lines[0].len() as i32{
                    let other_char = map.get(&(x2,y2)).unwrap(); 

                    if *other_char == '.' {continue;}

                    if current_char == other_char{
                        let antinode_pos = (x + (x-x2), y + (y-y2));
                        if antinode_pos.0 < 0 || antinode_pos.0 >= split_lines[0].len() as i32
                            || antinode_pos.1 < 0 || antinode_pos.1 >= split_lines.len() as i32{ 
                                continue;
                            }
                        
                        if x == x2 && y == y2{
                            continue;
                        }

                            //println!("{} {}: {}, {} {}: {}", x,y,current_char, x2,y2,other_char); 
                            //println!("{:?}", antinode_pos);
                            unique_antinodes.insert(antinode_pos);

                    }

                }
            }


        }
    }


    println!("Part 1 Answer = {}", unique_antinodes.len());
}

struct GridNode{
    node_char: char,
    inline_nodes: HashSet<(i32, i32)>
}

fn part2(contents: &str)
{
    let split_lines: Vec<&str> = contents.split('\n').collect();
    let mut map: HashMap<(i32, i32), char> = split_lines.iter()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars()
        .enumerate() // Get index (column number)
        .map(move |(x, c)| ((x as i32, y as i32), c))
    }).collect();

    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut nodes: HashMap<(i32,i32), GridNode> = HashMap::new();
    for y in 0..split_lines.len() as i32{
        for x in 0..split_lines[0].len() as i32{
            let current_char = map.get(&(x,y)).unwrap();
            if *current_char == '.' {continue;}

            nodes.insert((x,y), GridNode{node_char: *current_char, inline_nodes: HashSet::new()});

            for y2 in 0..split_lines.len() as i32{
                for x2 in 0..split_lines[0].len() as i32{
                    let other_char = map.get(&(x2,y2)).unwrap(); 

                    if *other_char == '.' {continue;}

                    if current_char == other_char{
                        //let antinode_pos = (x + (x-x2), y + (y-y2));
                        
                        if x == x2 && y == y2{
                            continue;
                        }

                        let node = nodes.get_mut(&(x,y)).unwrap();
                        node.inline_nodes.insert((x2,y2));
                            //println!("{} {}: {}, {} {}: {}", x,y,current_char, x2,y2,other_char); 
                            //println!("{:?}", antinode_pos);
                            //unique_antinodes.insert(antinode_pos);

                    }

                }
            }

        }
    }

    for y in 0..split_lines.len() as i32{
        for x in 0..split_lines[0].len() as i32{

            if let Some(node) = nodes.get(&(x,y)){

                for other_node in node.inline_nodes.clone().into_iter(){

                    let diff = (x-other_node.0, y-other_node.1);

                    let mut end = false;
                    let mut last_pos = (x,y);
                    while !end{
                        let antinode_pos = (last_pos.0 + diff.0, last_pos.1 + diff.1);

                        if node.inline_nodes.len() >= 1 {
                            unique_antinodes.insert((x,y));
                            //end = true;
                        }
                        if antinode_pos.0 < 0 || antinode_pos.0 >= split_lines[0].len() as i32
                            || antinode_pos.1 < 0 || antinode_pos.1 >= split_lines.len() as i32{ 
                                end = true;
                                continue;
                        }

                        unique_antinodes.insert(antinode_pos);
                        last_pos = antinode_pos;

                    }



                }
            }

        }
    }



    for y in 0..split_lines.len() as i32{
        for x in 0..split_lines[0].len() as i32{
            let current_char = map.get(&(x,y)).unwrap();

            if unique_antinodes.contains(&(x,y))
            {
                print!("#");
            } 
            else {
                print!("{}", current_char);
            }
        
        }
        println!("");
    }
    println!("Part 2 Answer = {}", unique_antinodes.len());
}