use std::fs;
use regex::Regex;
use substring::Substring;

struct CaptureData{
    capture_string: String,
    capture_pos: usize,
    capture_line: usize,
    capture_len: usize,
}

fn main(){
    let contents = fs::read_to_string("./inputs/intput3.txt").expect("Test");
    part1(&contents);
    part2(&contents);

}

fn part1(contents: &str)
{
    let mut capture_list: Vec<CaptureData> = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        let re2 = Regex::new(r"[^0-9a-zA-z\s]?\d+[^0-9a-zA-Z\s]?").unwrap();
        let captures2: Vec<&str> = re2.find_iter(line).map(|capture| capture.as_str()).collect();

        for capture in captures2
        {
            let mut data: CaptureData = CaptureData { capture_string: "".to_owned() , capture_pos: 0, capture_line: 0, capture_len: 0 };
            data.capture_string = capture.to_owned();
            data.capture_pos = line.find(capture).unwrap_or(0);
            data.capture_len = capture.len();
            data.capture_line = i;

            capture_list.push(data);
        }
    }


    let lines: Vec<&str> = contents.lines().collect();
    let re2 = Regex::new(r"([^0-9.]){1}").unwrap();
    let mut total = 0;
    for (_i, capture) in capture_list.iter().enumerate(){
        let line = lines[capture.capture_line];
        let before = if capture.capture_line > 0 {lines[capture.capture_line-1]} else {""};
        let after = if capture.capture_line < lines.len()-1 {lines[capture.capture_line+1]} else {""};

        let mut check_start = capture.capture_pos;
        if check_start > 0
        {
            let s = line.chars().nth(check_start).unwrap();
            let re2 = Regex::new(r"([0-9]){1}").unwrap();
            if re2.is_match(&s.to_string())
            {
                //println!("{}", &s.to_string());
                check_start -= 1;
            }
        }
        let check_end = capture.capture_pos + capture.capture_len;
        
        //println!("{} {checkStart} {checkEnd} {}", capture.capture_string, line.len().to_string());

        let sub1 = before.substring(check_start, check_end);
        let sub2 = line.substring(check_start, check_end);
        let sub3 = after.substring(check_start, check_end);
        //println!("{}", sub2);

        let symbols1: Vec<&str> = re2.find_iter(sub1).map(|s| s.as_str()).collect();
        let symbols2: Vec<&str> = re2.find_iter(sub2).map(|s| s.as_str()).collect();
        let symbols3: Vec<&str> = re2.find_iter(sub3).map(|s| s.as_str()).collect();
        

        if symbols1.len() > 0 || symbols2.len() > 0 || symbols3.len() > 0
        {
            let remove_regex = Regex::new(r"[^0-9a-zA-Z]").unwrap();
            let number = remove_regex.replace_all(capture.capture_string.as_str(), "");
            total += number.parse::<i32>().unwrap();
        }
    }

    println!("Part 1 Answer = {total}");

}


fn part2(contents: &str)
{
    let mut capture_list: Vec<CaptureData> = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        let re2 = Regex::new(r"[*]").unwrap();
        let captures2: Vec<&str> = re2.find_iter(line).map(|capture| capture.as_str()).collect();

        let mut line_sub_start = 0;
        for capture in captures2
        {
            let line_sub = line.substring(line_sub_start, line.len());
            //println!("{}", lineSub);
             
            let mut data: CaptureData = CaptureData { capture_string: "".to_owned() , capture_pos: 0, capture_line: 0, capture_len: 0 };
            data.capture_string = capture.to_owned();
            data.capture_pos = line_sub_start + line_sub.find(capture).unwrap_or(0);
            data.capture_len = capture.len();
            data.capture_line = i;
            
            line_sub_start = data.capture_pos+1;
            //println!("{}, {}, {}, {}", data.capture_string, data.capture_line, data.capture_pos, data.capture_len);
            capture_list.push(data);
        }
    }

    
    //println!("{}", capture_list.len().to_string());
    let mut num_list: Vec<CaptureData> = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        let re2 = Regex::new(r"\d+").unwrap();
        let captures2: Vec<&str> = re2.find_iter(line).map(|capture| capture.as_str()).collect();

        let mut line_sub_start = 0;
        for capture in captures2
        {
            let line_sub = line.substring(line_sub_start, line.len());
            let mut data: CaptureData = CaptureData { capture_string: "".to_owned() , capture_pos: 0, capture_line: 0, capture_len: 0 };
            data.capture_string = capture.to_owned();
            data.capture_pos = line_sub_start + line_sub.find(capture).unwrap_or(0);
            data.capture_len = capture.len();
            data.capture_line = i;

            line_sub_start = data.capture_pos + data.capture_len;
           // println!("{} {} {} {lineSubStart}", data.capture_string, data.capture_pos.to_string(), data.capture_len.to_string());
            num_list.push(data);
        }
    }


    let lines: Vec<&str> = contents.lines().collect();
    let re2 = Regex::new(r"([0-9]){1}").unwrap();
    let mut total = 0;
    for capture in capture_list
    {
        let line = lines[capture.capture_line];
        let before = if capture.capture_line > 0 {lines[capture.capture_line-1]} else {""};
        let after = if capture.capture_line < lines.len()-1 {lines[capture.capture_line+1]} else {""};

        let check_start = capture.capture_pos;
        let check_end = capture.capture_pos + capture.capture_len;

 
        
        //println!("{} {checkStart} {checkEnd} {} {}", capture.capture_string, line.len().to_string(), capture.capture_line.to_string());
        let array: Vec<Vec<char>> = [before.chars().collect(), line.chars().collect(), after.chars().collect()].to_vec();
        let mut num_coords1: Vec<(usize, usize)> = Vec::new();

        let mut l = 0;
        for line in array
        {
            let mut c = 0;
            let mut just_found = false;
            for char in line
            {
                if just_found 
                {
                    if !re2.is_match(&char.to_string())
                    {
                        //println!("found {}", char.to_string());
                        just_found = false;
                    }
                }
                else if re2.is_match(&char.to_string())
                {
                    //println!("{l},{c} {}", &char.to_string()); 
                    if c >= check_start-1 && c < check_end+1
                    {
                        //println!("{l},{c}");
                        num_coords1.push((l,c));
                        just_found = true;
                    }
                }
                c+=1;
            }
            l+=1;
        }

        //println!("{}", numCoords1.len().to_string());
        
        let mut w: Vec<u32> = Vec::new();
        for i in 0..num_coords1.len()
        {
            for number in num_list.iter()
            {
                //println!("{}", number.capture_string);
                let check_start = number.capture_pos;
                let check_end = number.capture_pos + number.capture_len;

                let line_no = capture.capture_line + num_coords1[i].0 - 1;
                if line_no == number.capture_line
                {
                    if num_coords1[i].1 >= check_start && num_coords1[i].1 < check_end
                    {
                        //println!("{checkStart}, {checkEnd}");
                        //println!("{}", number.capture_string);
                        w.push(number.capture_string.parse::<u32>().unwrap());
                    }
                }

            }
        }

        let mut gear_ratio = 1;
        if w.len() > 1 && w.len() < 3
        {
            for num in w.iter()
            {
                //println!("{} {num} {gear_ratio} {}", w.len().to_string(), (num*gear_ratio).to_string());
            
                gear_ratio *= num;
            }
            total += gear_ratio;
        }
    }


    println!("Part 2 Answer = {total}");

}

