use core::str;
use std::{collections::HashMap, fs, i32};

fn main() {
    println!("Day11:");
    
    let contents = fs::read_to_string("./inputs/input11.txt").expect("Not Found");

    //part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in &lines {
        let key_values: Vec<&str> = l.split(':').collect();
        let key: &str = key_values[0];

        let values: Vec<&str> = key_values[1].split(' ').filter(|f| !f.is_empty()).collect();
        map.insert(key, values);
    }

    let start: &str = "you";
    let paths = traverse(start, start, "out", &map, &["you", "out"].to_vec());


    println!("Part 1 Answer = {}", paths);
}

fn part2(contents: &str)
{
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut reverse_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in &lines {
        let key_values: Vec<&str> = l.split(':').collect();
        let key: &str = key_values[0];

        let values: Vec<&str> = key_values[1].split(' ').filter(|f| !f.is_empty()).collect();
        map.insert(key, values.clone());

        for v in values {
            reverse_map.entry(v).or_insert_with(Vec::new).push(key);
        }
    }

    //for (k, v) in &reverse_map {
    //    println!("{} - {:?}", k, v);
    //}

    let start: &str = "svr";
    let check_vals: Vec<&str> = ["dac", "fft"].to_vec(); 
    let mut path_vals: HashMap<(String, String), u64> = HashMap::new();
    //let paths = traverse("dac", "dac", "out", &map, &check_vals);
    //let paths = traverse_2(start, &map, &check_vals, vec![], &mut path_vals);
    let check_vals: Vec<&str> = ["out", "dac", "fft", "svr"].to_vec();
    let mut paths = 1;
    for i in 0..check_vals.len()-1 {
        //println!("Checking {} -> {}", check_vals[i+1], check_vals[i]);
        paths *= traverse_3(check_vals[i+1], check_vals[i+1], check_vals[i], &map, &check_vals, &mut path_vals);
    }

    let mut paths2 = 1; 
    let check_vals2: Vec<&str> = ["out", "fft", "dac", "svr"].to_vec();
    let mut path_vals2: HashMap<(String, String), u64> = HashMap::new();
    for i in 0..check_vals2.len()-1 {
        //println!("Checking {} -> {}", check_vals2[i+1], check_vals2[i]);
        paths2 *= traverse_3(check_vals2[i+1], check_vals2[i+1], check_vals2[i], &map, &check_vals2, &mut path_vals);
    }


    //paths = traverse_3("svr", "svr", "out", &map, &check_vals, &mut path_vals);

    println!("Part 2 Answer = {}", paths + paths2);
}

fn traverse(current: &str, start: &str, val_to_find: &str, map: &HashMap<&str, Vec<&str>>, check_vals: &Vec<&str>) -> i32 {
    if current == val_to_find {
        return 1;
    }

    if current != val_to_find && current != start {
        if check_vals.contains(&current) {
            return 0;
        }
    }

    //println!("{}", current);
    let next_vals = map.get(current).unwrap();
    let mut paths: i32 = 0;
    for val in next_vals {
        paths += traverse(val, start, val_to_find, map, check_vals);
    }
        
    return paths;
} 

fn traverse_save(current: &str, start: &str, val_to_find: &str, map: &HashMap<&str, Vec<&str>>, check_vals: &Vec<&str>, path: &mut HashMap<String, i32>) -> i32 {
    if current == val_to_find {
        return 1;
    }

    if current != val_to_find && current != start {
        if check_vals.contains(&current) {
            return 0;
        }
    }
    
    if path.contains_key(&current.to_string()) {
        return *path.get(&current.to_string()).unwrap();
    }

    //println!("{}", current);
    let next_vals = map.get(current).unwrap();
    let mut paths: i32 = 0;
    for val in next_vals {
        paths += traverse(val, start, val_to_find, map, check_vals);
    }
    
    path.insert(current.to_string(), paths);

    return paths;
} 

fn traverse_2(current: &str, map: &HashMap<&str, Vec<&str>>, check_vals: &Vec<&str>, has_hit: Vec<&str>, path: &mut HashMap<String, i32>) -> i32 {
    //println!("{} {}", current, has_hit.len());
    if current == "out" {
        if has_hit.contains(&"dac") && has_hit.contains(&"fft") {
            //println!("HERE");
            return 1;
        }
        else {
            return 0;
        }
    }

    //println!("SH");

    //if path.contains_key(&current.to_string()) {
    //    println!("{} - {}", current, has_hit.len());
    //    return *path.get(&current.to_string()).unwrap();
    //}

    //println!(" HAS HIT LEN {}", has_hit.len());
    let mut hit = has_hit.clone();
    //println!("HIT LEN {}, HAS HIT LEN {}", hit.len(), has_hit.len());
    if check_vals.contains(&current) {
        hit.push(current);
        //println!("PUSH {} - {}", current, hit.len());
    }

    //println!("{} {}", current, hit.len());

    let next_vals = map.get(current).unwrap();
    let mut paths: i32 = 0;
    for &val in next_vals {
        paths += traverse_2(val, map, check_vals, hit.clone(), path);
        //println!("HERE val {}, {}", val, paths);
    }

    //path.insert(current.to_string(), paths);
    return paths;
}

fn traverse_3(current: &str, start: &str, val_to_find: &str, map: &HashMap<&str, Vec<&str>>, check_vals: &Vec<&str>, path: &mut HashMap<(String, String), u64>) -> u64 {
    if current == val_to_find {
        return 1;
    }

    if let Some(&paths) = path.get(&(current.to_string(), val_to_find.to_string())) {
        return paths;
    }
        
    
    let mut paths: u64 = 0;
    if let Some(next_vals) = map.get(current) {
        for &val in next_vals {
            paths += traverse_3(val, start, val_to_find, map, check_vals, path);
        }
    }

    println!("{}", paths);
    path.insert((current.to_string(), val_to_find.to_string()), paths);


    return paths;
}