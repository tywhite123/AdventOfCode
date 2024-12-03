use std::fs;

fn main() {
    println!("Day 02:");
    
    let contents = fs::read_to_string("./inputs/input02.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();

    let mut safe = 0;
    for report in split_values{
        let values : Vec<u32> = report.split(' ')
            .enumerate()
            .map(|f| f.1.parse::<u32>().unwrap())
            .collect();
        
        let mut is_safe = true;

        let mut last = -1; 
        for v in values.windows(2){
            let current = v[0] as i32;
            let next = v[1] as i32;
            let diff = (current - next) as i32;

            if diff.abs() > 3 || diff.abs() < 1{
                is_safe = false;
            }

            if last != -1 {
                let last_diff = last - current;
                if (last_diff < 0 && diff > 0) ||
                    (last_diff > 0 && diff < 0)
                {
                    is_safe = false;
                }
            }

            last = current;
        }

        if is_safe { safe+=1; }
    }
    

    println!("Part 1 Answer = {}", safe);
}

fn part2(contents: &str)
{
    let split_values : Vec<&str> = contents.split('\n').collect();

    let mut safe = 0;
    for report in split_values{
        let values : Vec<u32> = report.split(' ')
            .enumerate()
            .map(|f| f.1.parse::<u32>().unwrap())
            .collect();
        
        let mut is_safe = true;

        let mut last = -1;
        let mut dampned = false;
        for v in 0..values.len()-1{
            let current = values[v] as i32;
            let next = values[v+1] as i32;
            let mut diff = (current - next) as i32;

            println!("{} {} {}", last, current, next);
            if !dampned
            {
                if diff.abs() > 3 || diff.abs() < 1{
                    dampned = true;
                    diff = last - current;
                    println!("here");
                }
                if last != -1 {
                    let last_diff = last - current;
                    if last_diff.abs() > 3 || last_diff.abs() < 1{
                        dampned = true;
                        diff = last - current;
                        println!("here");
                    }
                    if (last_diff < 0 && diff > 0) ||
                        (last_diff > 0 && diff < 0)
                    {
                        dampned = true;
                        println!("here");
                        continue;
                    }
                }
            }

            if diff.abs() > 3 || diff.abs() < 1{
                is_safe = false;
            }

            if last != -1 {
                let last_diff = last - current;

                if (last_diff < 0 && diff > 0) ||
                    (last_diff > 0 && diff < 0)
                {
                    is_safe = false;
                }
            }

            last = current;
        }

        if is_safe { safe+=1; println!("{:?}", values)}
    }
    
    println!("Part 2 Answer = {}", safe);
}