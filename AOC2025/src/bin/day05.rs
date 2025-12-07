use std::{collections::{BTreeSet, HashSet}, fs};

fn main() {
    println!("Day05:");
    
    let contents = fs::read_to_string("./inputs/input05.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let splits: Vec<&str> = contents.split("\n\n").collect();
    let ranges: Vec<(i64, i64)> = splits[0].split('\n').map(|f| {
        let ids: Vec<i64> = f.split('-').map(|c| c.parse::<i64>().unwrap()).collect();

        (ids[0], ids[1])
    }).collect();

    let values: Vec<i64> = splits[1].split('\n').map(|f| f.parse::<i64>().unwrap()).collect();

    let mut total_fresh = 0;
    for value in values {
        for range in ranges.clone() {
            if value >= range.0 && value <= range.1 {
                total_fresh += 1;
                break;
            }
        }
    }

    println!("Part 1 Answer = {}", total_fresh);
}

fn part2(contents: &str)
{
    let splits: Vec<&str> = contents.split("\n\n").collect();
    let mut ranges: Vec<(i64, i64)> = splits[0].split('\n').map(|f| {
        let ids: Vec<i64> = f.split('-').map(|c| c.parse::<i64>().unwrap()).collect();

        (ids[0], ids[1])
    }).collect();

    let values: Vec<i64> = splits[1].split('\n').map(|f| f.parse::<i64>().unwrap()).collect();

    //let mut fresh_ranges : BTreeSet<(i64, i64)> = BTreeSet::new();
    //for value in values {
    //    for range in ranges.clone() {
    //        if value >= range.0 && value <= range.1 {
    //            fresh_ranges.insert(range);
    //            //println!("{} {}", range.0, range.1);
    //        }
    //    }
    //}

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = vec![];
    for r in ranges {
        if let Some(last) = merged.last_mut(){
            if r.0 <= last.1+1 {
                last.1 = last.1.max(r.1);
            }
            else {
                merged.push(r);
            }
        }
        else {
            merged.push(r);
        }
    }

    //while true {

        //let mut end_merge = 0;
        //let mut merged_ranges: BTreeSet<(i64, i64)> = BTreeSet::new();
        ////if end_merge+1 >= fresh_ranges.len() {
        ////    end_merge = 0;
        ////}

        //for i in 0..fresh_ranges.len()-1 {
        //    if i < end_merge {
        //        continue;
        //    }
        //    let range1 = fresh_ranges[i];
        //    let mut new_range: (i64, i64) = range1;
        //    for j in i+1..fresh_ranges.len() {
        //        if i == j { continue; }
        //        let range2 = fresh_ranges[j];
        //        if overlaps(new_range, range2) { 
        //            if range2.0 < new_range.0 {
        //                new_range.0 = range2.0;
        //            }

        //            if new_range.1 <= range2.1 {
        //                new_range.1 = range2.1;
        //            }

        //            end_merge = j;
        //        } 
        //    }
        //    merged_ranges.insert(new_range);
        //}
        

        //fresh_ranges = merged_ranges.iter().cloned().collect();
        //merged_ranges.clear();

        //if print == false {
        //    break;
        //}
    //}


    let mut total_fresh = 0;
    for range in &merged {
        println!("{} {}", range.0, range.1);
        total_fresh += range.1 - range.0 + 1;    
    }

    println!("Part 2 Answer = {}", total_fresh);
}

fn overlaps(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}