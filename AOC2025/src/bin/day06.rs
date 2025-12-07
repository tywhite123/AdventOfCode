use core::num;
use std::fs;

fn main() {
    println!("Day06:");
    
    let contents = fs::read_to_string("./inputs/input06.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let rows: Vec<Vec<&str>> = contents.split('\n').map(|f| f.split_whitespace().collect()).collect();

    let mut total: i64 = 0;
    for i in 0..rows[0].len()
    {
        let mut nums: Vec<i64> = vec![];
        for j in 0..rows.len() {
            if j == rows.len()-1 {
                let sign = rows[j][i];
                let mut row_total = 0;
                for k in 0..nums.len() {
                    //print!("{} {}", sign, nums[k]);
                    if sign == "*" {
                        if row_total == 0 { row_total = 1;}
                        row_total *= nums[k];
                    }
                    else {
                        row_total += nums[k];
                    }
                }
                total += row_total;
                //println!(" = {}\n", row_total);
            }
            else {
                let num: i64 = rows[j][i].parse::<i64>().unwrap();
                nums.push(num);
            }
        }
    }
    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let mut rows: Vec<&str> = contents.split("\n").collect();

    let mut sign_row: Vec<char> = rows.pop().unwrap().chars().filter(|f| !f.is_whitespace()).collect();
    let rows_len = rows.len()-1;

    

    let mut cols : Vec<Vec<char>> = {
        let row_vec: Vec<Vec<char>> = rows.iter().map(|c| c.chars().collect()).collect();
        let width = row_vec[0].len();

        (0..width).map(|i| row_vec.iter().map(|row| row[i]).collect())
        .collect()

    };

    let mut total = 0;
    let mut row_total = 0;
    let mut sign_idx = 0;
    for col in 0..cols.len() {
        let column = &cols[col];
        let mut full_num: String = String::new();
        for row in 0..column.len() {
            if column[row].is_numeric() {
                full_num.push(column[row]);
            }
        }

        if full_num == ""{
            //println!("{}\n", row_total);
            total += row_total;
            row_total = 0;
            sign_idx += 1;
        }
        else {
            //println!("{}", full_num);
            let parsed : i64 = full_num.parse::<i64>().unwrap();
            let sign = sign_row[sign_idx];

            if sign == '*' {
                if row_total == 0 { row_total = 1;}
                row_total *= parsed;
            }
            else {
                row_total += parsed;
            }
        }

        if col == cols.len()-1 {
            total += row_total;
        }
    }



    println!("Part 2 Answer = {}", total);
}