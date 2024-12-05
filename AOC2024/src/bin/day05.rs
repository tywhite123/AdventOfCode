use std::fs;

fn main() {
    println!("Day 05:");
    
    let contents = fs::read_to_string("./inputs/input05.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let (page_order, pages) = contents.split_once("\n\n").unwrap();

    let split_page_order: Vec<&str> = page_order.split("\n").collect();
    let updates: Vec<&str> = pages.split("\n").collect();


    let page_order_vals: Vec<(i32, i32)> = split_page_order.iter().map(|f| {
        let item: Vec<i32>  = f.split("|").flat_map(|c| c.parse::<i32>()).collect();

        (item[0], item[1])
    }).collect();

    let total_correct = updates.iter().fold(0, |acc, f| {
        let val : Vec<i32> = f.split(",").flat_map(|c| c.parse::<i32>()).collect();

        let mut use_update = true; 
        page_order_vals.iter().for_each(|(page, check)|{
            if val.contains(page) {
                if let Some(pos) = val.iter().position(|x| x == check) {
                    if !val.iter().take(pos).any(|x| x == page)
                    {
                        use_update = false;
                    }
                }
            }
        });
        let mut increase = 0;
        if use_update {
            increase = val[val.len()/2];
        }
        acc + increase
    });
    

    println!("Part 1 Answer = {}", total_correct);
}

fn part2(contents: &str)
{
    let (page_order, pages) = contents.split_once("\n\n").unwrap();

    let split_page_order: Vec<&str> = page_order.split("\n").collect();
    let updates: Vec<&str> = pages.split("\n").collect();


    let page_order_vals: Vec<(i32, i32)> = split_page_order.iter().map(|f| {
        let item: Vec<i32>  = f.split("|").flat_map(|c| c.parse::<i32>()).collect();

        (item[0], item[1])
    }).collect();

    let total_correct = updates.iter().fold(0, |acc, f| {
        let mut val : Vec<i32> = f.split(",").flat_map(|c| c.parse::<i32>()).collect();

        let mut use_update = false;
        let mut still_updating = true;
        while still_updating {
            still_updating = false; 
            page_order_vals.iter().for_each(|(page, check)|{
                if val.contains(page) && val.contains(check){
                    if let Some(pos) = val.iter().position(|x| x == check) {
                        if !val.iter().take(pos).any(|x| x == page)
                        {
                            if let Some(pos2) = val.iter().position(|x| x == page){
                                val.remove(pos);
                                val.insert(pos2, *check);
                                use_update = true;
                                still_updating = true;
                            } 
                        }
                    }
                }
            });
        }
        let mut increase = 0;
        if use_update {
            increase = val[val.len()/2];
        }
        acc + increase
    });

    println!("Part 2 Answer = {}", total_correct);
}