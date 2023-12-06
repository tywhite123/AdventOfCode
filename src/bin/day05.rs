use std::fs;


struct Map
{
    name: String,
    entries: Vec<MapEntry>
}

struct MapEntry
{
    dest: u64,
    src: u64,
    len: u64
}

fn main()
{

    println!("Day 5:");  
    
    //let contents = fs::read_to_string("./inputs/test4.txt").expect("Not Found");
    let contents = fs::read_to_string("./inputs/input5.txt").expect("Not Found");
    
    //println!("{}", contents);

    part1(&contents);
    part2(&contents);

}

fn part1(contents: &str)
{
    let mut maps: Vec<Map> = Vec::new();

    let lines_to_process: Vec<&str> = contents.lines().filter(|s| !s.is_empty()).collect();
    //let splits: Vec<&str> = contents.lines().collect();
    let mut line_iter = lines_to_process.iter();
    let seeds_line: Vec<&str>= line_iter.next().unwrap().split(':').collect();
    let mut seed_iter = seeds_line.iter();
    let _seed_header = seed_iter.next().unwrap();

    let seeds: Vec<u32> = seed_iter.next()
                            .unwrap()
                            .split_whitespace()
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect();


    //println!("Split = {}", );

    let mut current_map = 0;
    for s in line_iter
    {
        if s.contains(":")
        {
            let mut map: Map = Map { name: "".to_string(), entries: Vec::new() };
            map.name = s.to_string();
            if maps.len() > 0
            {
                current_map += 1;
            }
            maps.push(map);
        }
        else 
        {
            let mut entry: MapEntry = MapEntry { dest: 0, src: 0, len: 0 };
            let splits: Vec<u64> = s.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

            entry.dest = splits[0];
            entry.src = splits[1];
            entry.len = splits[2];

            //println!("{} {} {}", entry.dest, entry.src, entry.len);
            maps[current_map].entries.push(entry);
        }
    }

    let mut lowest_location = u64::MAX;
    for seed in seeds
    { 
        //println!("Current Seed = {}", seed);
        let mut current_location: u64 = seed.into();
        for map in 0..maps.len()
        {
            //println!("Current map: {}", maps[map].name);
            for entry in 0..maps[map].entries.len()
            {
                let mapEntry = &maps[map].entries[entry];
                //println!("Current src = {}, src + len = {}", mapEntry.src.to_string(), (mapEntry.src+mapEntry.len).to_string());
                let srcLen = mapEntry.src+mapEntry.len;
                if current_location >= mapEntry.src && current_location < srcLen
                {
                    let diff = current_location - mapEntry.src;
                    current_location = mapEntry.dest + diff;
                    //println!("Current location = {current_location}, diff = {diff}, dest = {}", mapEntry.dest.to_string());
                    break;
                }
            }
        }
        
        if lowest_location > current_location
        {
            lowest_location = current_location;
        }
    }
    
    println!("Part 1 Answer = {}", lowest_location);
}

fn part2(contents: &str)
{
    let mut maps: Vec<Map> = Vec::new();

    let lines_to_process: Vec<&str> = contents.lines().filter(|s| !s.is_empty()).collect();
    //let splits: Vec<&str> = contents.lines().collect();
    let mut line_iter = lines_to_process.iter();
    let seeds_line: Vec<&str>= line_iter.next().unwrap().split(':').collect();
    let mut seed_iter = seeds_line.iter();
    let _seed_header = seed_iter.next().unwrap();

    let mut seeds: Vec<u64> = seed_iter.next()
                            .unwrap()
                            .split_whitespace()
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect();


    //println!("Split = {}", );

    let mut current_map = 0;
    for s in line_iter
    {
        if s.contains(":")
        {
            let mut map: Map = Map { name: "".to_string(), entries: Vec::new() };
            map.name = s.to_string();
            if maps.len() > 0
            {
                current_map += 1;
            }
            maps.push(map);
        }
        else 
        {
            let mut entry: MapEntry = MapEntry { dest: 0, src: 0, len: 0 };
            let splits: Vec<u64> = s.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

            entry.dest = splits[0];
            entry.src = splits[1];
            entry.len = splits[2];

            //println!("{} {} {}", entry.dest, entry.src, entry.len);
            maps[current_map].entries.push(entry);
        }
    }

    let mut lowest_location = u64::MAX;
    // for seed_start in (0..seeds.len()).step_by(2)
    // { 
    //     let seed_range_len = seeds[seed_start+1];
    //     for current_seed in 0..seed_range_len
    //     {
    //         let seed: u64 = seeds[seed_start] + current_seed;
    //         println!("Current Seed = {}", seed);
    //         let mut current_location: u64 = seed;
    //         for map in 0..maps.len()
    //         {
    //             //println!("Current map: {}", maps[map].name);
    //             for entry in 0..maps[map].entries.len()
    //             {
    //                 let mapEntry = &maps[map].entries[entry];
    //                 //println!("Current src = {}, src + len = {}", mapEntry.src.to_string(), (mapEntry.src+mapEntry.len).to_string());
    //                 let srcLen = mapEntry.src+mapEntry.len;
    //                 if current_location >= mapEntry.src && current_location < srcLen
    //                 {
    //                     let diff = current_location - mapEntry.src;
    //                     current_location = mapEntry.dest + diff;
    //                     //println!("Current location = {current_location}, diff = {diff}, dest = {}", mapEntry.dest.to_string());
    //                     break;
    //                 }
    //             }
    //         }
        
    //         if lowest_location > current_location
    //         {
    //             lowest_location = current_location;
    //         }
    //     }
    // }

    for map in 0..maps.len()
    {
        //println!("{}", maps[map].name);
        seeds = create_new_ranges(seeds, &maps[map]);
    }

    for seed in (0..seeds.len()).step_by(2)
    {
        if seeds[seed] < lowest_location
        {
            lowest_location = seeds[seed];
        }
        //println!("Seed = {}, Range = {}", seeds[seed], seeds[seed+1]);
    }

    println!("Part 2 Answer = {lowest_location}");
}

fn create_new_ranges(mut oldRanges: Vec<u64>, map: &Map) -> Vec<u64>
{
    let mut newRanges: Vec<u64> = Vec::new();

    //println!("{}", newRanges.len());
    let mut finished = false;
    let mut i = 0;
    while !finished
    //for i in (0..oldRanges.len()).step_by(2)
    {
        let min = oldRanges[i];
        let max = min + oldRanges[i+1];

        //println!("{min}, {max}");

        let mut found = false;

        for n in 0..map.entries.len()
        {
            let entry = &map.entries[n];

            //println!("{} {}", entry.src, entry.src+entry.len);
            
            if max < entry.src
            {
                //println!("Here min = {} max = {} entry src = {}", min.to_string(), max.to_string(), entry.src.to_string());
                continue;
            }

            if min >= entry.src + entry.len
            {
                //println!("Here");
                continue;
            }

            if min >= entry.src && max < entry.src+entry.len
            {
                //println!("Here 1");
                let diff = min - entry.src;
                let destRangeStart = entry.dest + diff;
                let destRangeEnd = oldRanges[i+1];

                newRanges.push(destRangeStart);
                newRanges.push(destRangeEnd);

                found = true;
                break;
            }

            if min < entry.src && max >= entry.src+entry.len
            {
                //println!("Here 4");
                let destRangeStart = entry.dest;
                let destRangeEnd = (entry.dest + entry.len) - destRangeStart;

                oldRanges.push(min);
                oldRanges.push(entry.src-min);

                oldRanges.push(entry.src+entry.len);
                oldRanges.push(max - (entry.src+entry.len));

                newRanges.push(destRangeStart);
                newRanges.push(destRangeEnd);

                found = true;
                break;
            }

            if min >= entry.src && max >= entry.src+entry.len
            {
                //println!("Here 2");
                let diff = min - entry.src;
                let destRangeStart = entry.dest + diff;
                //println!("Min {} Src {} {}", oldRanges[i+1], entry.dest + entry.len, diff);
                let destRangeEnd = ((entry.dest + entry.len)) - destRangeStart;

                oldRanges.push(entry.src+entry.len);
                oldRanges.push(oldRanges[i+1] - destRangeEnd);

                newRanges.push(destRangeStart);
                newRanges.push(destRangeEnd);

                found = true;
                break;
            }

            if min < entry.src && max < entry.src
            {
                //println!("Here 3");
                let _diff = max - entry.src;
                let destRangeStart = entry.dest;
                let destRangeEnd = (entry.dest + entry.len) - destRangeStart;

                oldRanges.push(min);
                oldRanges.push(entry.src-min);

                newRanges.push(destRangeStart);
                newRanges.push(destRangeEnd);

                found = true;
                break;
            }

            //println!("Here\n min = {} max = {}\nsrc = {} src len = {}", min, max, entry.src, entry.src+entry.len);
        }

        if !found
        {
            //println!("Here 5");
            newRanges.push(oldRanges[i]);
            newRanges.push(oldRanges[i+1]);
        }

        i+=2;
        //println!("{i}");
        if i >= oldRanges.len()
        {
            //println!("Here 10");
            finished = true;
        }
    }

    //println!("new ranges len ={}", newRanges.len().to_string());

    return newRanges;
}

fn is_even(n: usize) -> bool
{
    n % 2 == 0
}