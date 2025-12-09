use std::{collections::HashSet, fs};

type Vector2 = (i64, i64);

fn main() {
    println!("Day09:");
    
    let contents = fs::read_to_string("./inputs/input09.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let coords: Vec<Vector2> = contents.split('\n').map(|f| {
        let parts: Vec<i64> = f.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        (parts[0], parts[1])
    }).collect();

    let mut max_area = 0;
    for a in 0..coords.len()-1 {
        for b in a+1..coords.len() {
            //if a == b { continue; }

            let w = (coords[a].0 - coords[b].0).abs()+1;
            let l = (coords[a].1 - coords[b].1).abs()+1;
            let area = w*l;
            //println!("A: {:?}, B: {:?}, Area: {}", coords[a], coords[b], area);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 1 Answer = {}", max_area);
}

fn part2(contents: &str)
{
    let coords: Vec<Vector2> = contents.split('\n').map(|f| {
        let parts: Vec<i64> = f.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        (parts[0], parts[1])
    }).collect();

    println!("Getting max vals");
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    for c in &coords {
        if c.0 > max_x {
            max_x = c.0;
        }

        if c.1 > max_y {
            max_y = c.1;
        }
    }

    max_x += 2;
    max_y += 2;

    println!("Setting up map and edges");
    let mut edges: HashSet<Vector2> = HashSet::new();

    let mut map: Vec<Vec<char>> = vec![vec!['.'; max_x as usize]; max_y as usize];

    //for c in 0..coords.len() {

    //    let next_coord = modulo((c+1) as i32, coords.len() as i32) as usize;
    //    let x_1 = coords[c].0.min(coords[next_coord].0);
    //    let y_1 = coords[c].1.min(coords[next_coord].1);
    //    let x_2 = coords[c].0.max(coords[next_coord].0);
    //    let y_2 = coords[c].1.max(coords[next_coord].1);

    //    for i in y_1..=y_2 {
    //        for j in x_1..=x_2 {
    //            if !coords.contains(&(j,i)){
    //                //map[i as usize][j as usize] = 'X';
    //                edges.insert((j, i));
    //                
    //            }
    //        }
    //    } 

    //    edges.insert(coords[c]);
    //    //map[coords[c].1 as usize][coords[c].0 as usize] = '#';
    //}

    //println!("Shading map");
    //for y in 0..max_y {
    //    for x in 0..max_x {
    //        //if map[y as usize][x as usize] == 'X' ||
    //        //    map[y as usize][x as usize] == '#' {
    //        //        continue;
    //        //    }
    //        
    //        if point_within_poly((x, y), &coords) {
    //            edges.insert((x, y));
    //            //map[y as usize][x as usize] = '/';
    //        }


    //    }
    //}

    //for y in 0..max_y {
    //    for x in 0..max_x {
    //        print!("{}", map[y as usize][x as usize]);
    //    }
    //    println!("")
    //}
    
    println!("Finding max area");
    let mut max_area = 0;
    for a in 0..coords.len()-1 {
        for b in a+1..coords.len() {
            //if a == b { continue; }
            //if !check_if_outside_any_corner(coords[a], coords[b], &coords, &edges) {
            //if !check_collisions(coords[a], coords[b], &coords){
                let w = (coords[a].0 - coords[b].0).abs()+1;
                let l = (coords[a].1 - coords[b].1).abs()+1;
                let area = w*l;
                //println!("A: {:?}, B: {:?}, Area: {}", coords[a], coords[b], area);

                if area > max_area {
                    if !check_if_outside_any_corner(coords[a], coords[b], &coords, &edges) 
                    {
                        max_area = area;
                        println!("{}", area)
                    }
                }
            //}
        }
    }
    println!("Part 2 Answer = {}", max_area);
}

fn check_if_outside_any_corner(a: Vector2, b: Vector2, coords: &Vec<Vector2>, edges: &HashSet<Vector2>) -> bool{
    let x_1 = a.0.min(b.0);
    let y_1 = a.1.min(b.1);
    let x_2 = a.0.max(b.0);
    let y_2 = a.1.max(b.1);

    let a_corner = (a.0, b.1);
    if !point_within_poly(a_corner, coords) {
        return true;
    }
    
    let b_corner = (b.0, a.1);
    if !point_within_poly(b_corner, coords) {
        return true;
    }

    for x in x_1..=x_2 {
        if !point_within_poly((x,y_1), coords) {
            return true;
        }
        if !point_within_poly((x,y_2), coords) {
            return true;
        }
    } 
    
    for y in y_1..=y_2 {
        if !point_within_poly((x_1,y), coords) {
            return true;
        }
        if !point_within_poly((x_2,y), coords) {
            return true;
        }
    } 

    //for y in y_1..=y_2 {
    //    for x in x_1..=x_2 {
    //        //if edges.contains(&(x,y)) {
    //        //    return false;
    //        //}
    //        if !point_within_poly((x,y), coords) {
    //            return true;
    //        }
    //        //if map[y as usize][x as usize] == '.' {
    //        //    return true;
    //        //}
    //    }
    //}

    return false;
}

fn is_left(a: Vector2, b: Vector2, c: Vector2) -> i64 {
   (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn point_on_segment(p: Vector2, a: Vector2, b: Vector2) -> bool {
    if is_left(a, b, p) != 0 {
        return false;
    }

    let min_x = a.0.min(b.0);
    let min_y = a.1.min(b.1);
    let max_x = a.0.max(b.0);
    let max_y = a.1.max(b.1);

    return p.0 >= min_x && p.0 <= max_x && p.1 >= min_y && p.1 <= max_y; 
}

fn point_within_poly(point: Vector2, coords: &Vec<Vector2>) -> bool{
    let mut inside = false;
    let mut j = coords.len()-1;

    for i in 0..coords.len(){
        //let j = modulo(((i as i32 )-1 + (coords.len() as i32)), coords.len() as i32) as usize;

        if point_on_segment(point, coords[j], coords[i])
        {
            if point == (0, 3) {
                println!("HERE");
            }
            return true;
        }

        if ray_intersects(point, coords[j], coords[i]) {
            inside = !inside;
        }

        j = i;
    }
    return inside;
}

fn modulo(a: i32, b: i32) -> i32
{
    return ((a % b) + b) % b;
}

fn ray_intersects(p: Vector2, a: Vector2, b: Vector2) -> bool {
    // Ensure a.y <= b.y
    let (a, b) = if a.1 <= b.1 { (a, b) } else { (b, a) };

    // Case where the ray is aligned with the vertex -> skip upper endpoint
    if /*p.1 == a.1 ||*/ p.1 == b.1 {
        return false;
    }

    // Does the ray intersect the vertical span?
    if p.1 < a.1 || p.1 > b.1 {
        return false;
    }

    // Intersection X using integer cross products (no division)
    // Equivalent to: p.0 < a.0 + (b.0 - a.0) * (p.1 - a.1) / (b.1 - a.1)
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let t = p.1 - a.1;

    // Compare: p.0 * dy < a.0 * dy + dx * t
    p.0 * dy < a.0 * dy + dx * t
}

fn check_collisions(a: Vector2, b: Vector2, coords: &Vec<Vector2>) -> bool{
    let mut j = coords.len()-1;

    let mut still_valid: bool = true;

    for i in 0..coords.len(){
        if (a.0 == coords[i].0 && a.1 == coords[i].1) ||
            (a.0 == coords[j].0 && a.1 == coords[j].1) ||
            (b.0 == coords[j].0 && b.1 == coords[j].1) ||
            (b.0 == coords[j].0 && b.1 == coords[j].1) {
                continue;
            }

        if aabb(as_box(a, b), as_box(coords[i], coords[j])) {
            println!("{:?} - {:?}, {:?} - {:?}", a, b, coords[i], coords[j]);
            if full_overlap((a, b), (coords[i], coords[j])) {
                //println!("full overlap");
                return true;
            }

            if partial_overlap((a,b), (coords[i], coords[j]))
            {
                //println!("partial overlap");
                still_valid = false;
            }

        }
        j = i;
    }
    //println!("no overlap");

    return still_valid;
}

fn as_box(a: Vector2, b: Vector2) -> (Vector2, Vector2) {
    let pos_x = a.0.min(b.0);
    let pos_y = a.1.min(b.1);
    let max_x = a.0.max(b.0);
    let max_y = a.1.max(b.1);
    let size_x = (max_x - pos_x).abs();
    let size_y = (max_y - pos_y).abs();

    return ((pos_x, pos_y), (size_x, size_y));
}

fn aabb(A: (Vector2, Vector2), B: (Vector2, Vector2)) -> bool {
    let x_col = A.0.0 + A.1.0 + 1 >= B.0.0 
        && B.0.0 + B.1.0 + 1 >= A.0.0;
    
    let y_col = A.0.1 + A.1.1 + 1 >= B.0.1 
        && B.0.1 + B.1.1 + 1 >= A.0.1;

    return x_col && y_col;
}

fn full_overlap(A: (Vector2, Vector2), B: (Vector2, Vector2)) -> bool {
    let a_min_x = A.0.0.min(A.1.0);
    let a_min_y = A.0.1.min(A.1.1);
    let a_max_x = A.0.0.max(A.1.0);
    let a_max_y = A.0.1.max(A.1.1);

    let b_min_x = B.0.0.min(B.1.0);
    let b_min_y = B.0.1.min(B.1.1);
    let b_max_x = B.0.0.max(B.1.0);
    let b_max_y = B.0.1.max(B.1.1);

    if b_min_x >= a_min_x && b_max_x <= a_max_x && 
        b_min_y >= a_min_y && b_max_y <= a_max_y {
            return true;
        }

    return false;
}

fn partial_overlap(A: (Vector2, Vector2), B: (Vector2, Vector2)) -> bool {
    let a_min_x = A.0.0.min(A.1.0);
    let a_min_y = A.0.1.min(A.1.1);
    let a_max_x = A.0.0.max(A.1.0);
    let a_max_y = A.0.1.max(A.1.1);

    let b_min_x = B.0.0.min(B.1.0);
    let b_min_y = B.0.1.min(B.1.1);
    let b_max_x = B.0.0.max(B.1.0);
    let b_max_y = B.0.1.max(B.1.1);

    if a_min_x <= b_max_x && a_max_x >= b_min_x && 
        a_min_y <= b_max_y && a_max_y >= b_min_y {
            return true;
        }
    
    return false;
}
