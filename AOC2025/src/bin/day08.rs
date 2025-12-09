use std::{collections::{HashMap, HashSet}, fs, i64, net::ToSocketAddrs, ops::IndexMut, vec};

type Vector3 = (i64, i64, i64);
type Circuit = Vec<Vector3>;

struct Connection {
    coord1: Vector3,
    coord2: Vector3,
    dist: f64,
}

fn main() {
    println!("Day08:");
    
    let contents = fs::read_to_string("./inputs/input08.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let coords: Vec<Vector3> = contents.split('\n').map(|f| {
        let parts: Vec<i64> = f.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        (parts[0], parts[1], parts[2])
    }).collect();

    let mut connection_distances: Vec<Connection> = vec![];
    let mut set: HashSet<(Vector3, Vector3)> = HashSet::new();
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if i == j {continue;}

            let dist = dist(coords[i], coords[j]);
            if set.contains(&(coords[i], coords[j])) || set.contains(&(coords[j], coords[i]))
            {
                continue;
            }

            connection_distances.push(Connection { coord1: coords[i], coord2: coords[j], dist });
            set.insert((coords[i], coords[j]));
        }
    }

    connection_distances.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(std::cmp::Ordering::Equal));

    //for c in &connection_distances {
    //    println!("Dist: {}, A: {:?}, B: {:?}", c.dist, c.coord1, c.coord2);
    //}

    let mut circuits: Vec<Circuit> = vec![];
    let mut set2: HashSet<Vector3> = HashSet::new();
    for i in 0..10 {
        let conn: &Connection = &connection_distances[i];
        
        if set2.contains(&conn.coord1) && set2.contains(&conn.coord2)
        {
            let mut first_found_idx = 0;
            for i in 0..circuits.len(){
                if circuits[i].contains(&conn.coord1) {
                    first_found_idx = i;
                    break;
                }
            }

            for i in 0..circuits.len() {
                if circuits[i].contains(&conn.coord2)
                {
                    if i > first_found_idx {
                        let mut old = circuits.remove(i);
                        circuits[first_found_idx].append(&mut old);
                        break;
                    }
                    else if i < first_found_idx {
                        let mut old = circuits.remove(first_found_idx);
                        circuits[i].append(&mut old);
                        break;
                    }
                }
            }
            continue;
            
        } else if set2.contains(&conn.coord1){
            for mut c in circuits.iter_mut() {
                if c.contains(&conn.coord1)
                {
                    c.push(conn.coord2);
                    set2.insert(conn.coord2);
                    //println!("HERE1");
                }
            }
        } else if set2.contains(&conn.coord2) {
            for mut c in circuits.iter_mut() {
                if c.contains(&conn.coord2)
                {
                    c.push(conn.coord1);
                    set2.insert(conn.coord1);
                    //println!("HERE2");
                }
            }
        } else {
            circuits.push(Circuit::new());
            let new_vec = circuits.last_mut().unwrap();
            new_vec.push(conn.coord1);
            new_vec.push(conn.coord2);
            set2.insert(conn.coord1);
            set2.insert(conn.coord2);
        } 

    }

    circuits.sort_by(|a,b| b.len().cmp(&a.len()));
    //circuits.reserve(0);

    let mut total = 1;
    for i in 0..3 {
        println!("{}", circuits[i].len());
        total *= circuits[i].len();
    }

    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let coords: Vec<Vector3> = contents.split('\n').map(|f| {
        let parts: Vec<i64> = f.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        (parts[0], parts[1], parts[2])
    }).collect();

    let mut connection_distances: Vec<Connection> = vec![];
    let mut set: HashSet<(Vector3, Vector3)> = HashSet::new();
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if i == j {continue;}

            let dist = dist(coords[i], coords[j]);
            if set.contains(&(coords[i], coords[j])) || set.contains(&(coords[j], coords[i]))
            {
                continue;
            }

            connection_distances.push(Connection { coord1: coords[i], coord2: coords[j], dist });
            set.insert((coords[i], coords[j]));
        }
    }

    connection_distances.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(std::cmp::Ordering::Equal));

    for c in &connection_distances {
        //println!("Dist: {}, A: {:?}, B: {:?}", c.dist, c.coord1, c.coord2);
    }

    let mut circuits: Vec<Circuit> = vec![];
    let mut set2: HashSet<Vector3> = HashSet::new();
    let mut f_val = 0;
    let mut prev_conn: Option<&Connection> = None;
    for i in 0..connection_distances.len() {
        let conn: &Connection = &connection_distances[i];
        //println!("{} - {:?} {:?}", circuits.len(), conn.coord1, conn.coord2);

        if prev_conn.is_some() {
            //println!("Prev {:?} {:?}", prev_conn.unwrap().coord1, prev_conn.unwrap().coord2);
        } 
        prev_conn = connection_distances.get(i);

        if set2.contains(&conn.coord1) && set2.contains(&conn.coord2)
        {
            let mut first_found_idx = 0;

            for i in 0..circuits.len(){
                if circuits[i].contains(&conn.coord1) {
                    first_found_idx = i;
                    break;
                }
            }

            for i in 0..circuits.len() {
                if circuits[i].contains(&conn.coord2)
                {
                    //f_val = conn.coord1.0 * conn.coord2.0; 
                    if i > first_found_idx {
                        let mut old = circuits.remove(i);
                        circuits[first_found_idx].append(&mut old);
                        if circuits[first_found_idx].len() == coords.len() {
                            f_val = conn.coord1.0 * conn.coord2.0;
                            println!("A: {:?}, B: {:?}", conn.coord1, conn.coord2);
                        }
                        break;
                    }
                    else if i < first_found_idx {
                        let mut old = circuits.remove(first_found_idx);
                        circuits[i].append(&mut old);

                        if circuits[i].len() == coords.len() {
                            f_val = conn.coord1.0 * conn.coord2.0; 
                            println!("A: {:?}, B: {:?}", conn.coord1, conn.coord2);
                        }
                        break;
                    }
                }
            }
            continue;
            
        } else if set2.contains(&conn.coord1){
            for mut c in circuits.iter_mut() {
                if c.contains(&conn.coord1)
                {
                    c.push(conn.coord2);
                    set2.insert(conn.coord2);
                    //println!("HERE1");
                }
                        
                if c.len() == coords.len() {
                    f_val = conn.coord1.0 * conn.coord2.0; 
                    println!("A: {:?}, B: {:?}", conn.coord1, conn.coord2);
                }
            }
        } else if set2.contains(&conn.coord2) {
            for mut c in circuits.iter_mut() {
                if c.contains(&conn.coord2)
                {
                    c.push(conn.coord1);
                    set2.insert(conn.coord1);
                    //println!("HERE2");
                }
                if c.len() == coords.len() {
                    f_val = conn.coord1.0 * conn.coord2.0; 
                    println!("A: {:?}, B: {:?}", conn.coord1, conn.coord2);
                }
            }
        } else {
            circuits.push(Circuit::new());
            let new_vec = circuits.last_mut().unwrap();
            new_vec.push(conn.coord1);
            new_vec.push(conn.coord2);
            set2.insert(conn.coord1);
            set2.insert(conn.coord2);
        } 

    }

    circuits.sort_by(|a,b| b.len().cmp(&a.len()));
    //circuits.reserve(0);

    //let mut total = 1;
    //for i in 0..3 {
    //    println!("{}", circuits[i].len());
    //    total *= circuits[i].len();
    //}

    println!("Part 2 Answer = {}", f_val);
}

fn dist((x1, y1, z1) : Vector3, (x2, y2, z2): Vector3) -> f64 {
    let val_to_root = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
    let rooted_val = (val_to_root as f64).sqrt();
    return rooted_val;
}