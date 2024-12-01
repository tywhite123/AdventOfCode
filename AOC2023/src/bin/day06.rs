use std::fs;

fn main()
{
    println!("Day 6:");  
    
    //let contents = fs::read_to_string("./inputs/test4.txt").expect("Not Found");
    let contents = fs::read_to_string("./inputs/input6.txt").expect("Not Found");
    
    println!("{}", contents);

    part1(&contents);
    part2(&contents);

}

fn part1(contents: &str)
{
    let mut lines = contents.lines();

    let mut times_section = lines.next().unwrap().split(':');
    let _times_heading = times_section.next().unwrap();

    let times: Vec<u32> = times_section.next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut distance_section = lines.next().unwrap().split(':');

    let _distance_heading = distance_section.next().unwrap();

    let distances: Vec<u32> = distance_section.next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();


    let mut records: Vec<u32> = Vec::new();
    for i in 0..times.len()
    {
        let mut rec = 0;
        for sec in 1..times[i]+1
        {
            let time_moving = times[i] - sec;
            let distance_moved = time_moving*sec;
            if distance_moved > distances[i]
            {
                rec+=1;
            }
            //println!("Button {} Movement {}", sec, time_moving);
        }
        records.push(rec);
    }

    let mut total = 0;
    for i in 0..records.len()
    {
        if i < 1
        {
            total += records[i];
        }
        else 
        {
            total *= records[i];    
        }
    }

    println!("Part 1 Answer = {total}");
}

fn part2(contents: &str)
{
    let mut lines = contents.lines();

    let mut times_section = lines.next().unwrap().split(':');
    let _times_heading = times_section.next().unwrap();

    let time_string: String = times_section.next()
        .unwrap()
        .split_whitespace()
        .collect();

    let time: u64 = time_string.parse::<u64>().unwrap();

    let mut distance_section = lines.next().unwrap().split(':');

    let _distance_heading = distance_section.next().unwrap();

    let distance_string: String = distance_section.next()
        .unwrap()
        .split_whitespace()
        .collect();

    
    //let distance: u64 = 2;
    let distance: u64 = distance_string.parse::<u64>().unwrap();

    println!("Time: {}, Dist: {}", time_string, distance_string);

    let mut rec = 0;
    for sec in 1..time
    {
        let time_moving: u64 = time - sec;
        let distance_moved: u64 = time_moving*sec;
        if distance_moved > distance
        {
            rec+=1;
        }
        //println!("Button {} Movement {}", sec, time_moving);
    }
    
    println!("Part 2 Answer = {rec}");
}
