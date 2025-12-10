use std::{fs, usize};

struct Machine {
    lights: u32,
    button_sets: Vec<u32>,
    joltage: Vec<i64>,
}

fn main() {
    println!("Day10:");
    
    let contents = fs::read_to_string("./inputs/input10.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let machines: Vec<Machine> = contents.split('\n').map(|f|{
        let splits: Vec<&str> = f.split(' ').collect();
        let indicators = splits[0];
        let joltage = splits[splits.len()-1];
        let buttons = splits[1..splits.len()-1].to_vec();

        let mut mac: Machine = Machine { lights: 0, button_sets: vec![], joltage: vec![] };
        let l: Vec<bool> = indicators.chars().filter(|c| c == &'#' || c == &'.').map(|c| match c {
            '#' => true,
            '.' => false,
            _ => false
        }).collect();
        for (i, s) in l.iter().enumerate() {
            if *s {
                mac.lights |= 1 << i;
            }
        }


        mac.button_sets = buttons.iter().map(|f| {
            let v = f.split(" ").flat_map(|b|{
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                
            }).collect::<Vec<i64>>();
            
            let mut mask: u32 = 0;

            for i in v {
                mask |= 1 << i
            }

            mask
        }).collect::<Vec<u32>>();

        mac.joltage = joltage.trim_matches(['{', '}']).split(',').map(|f| f.parse::<i64>().unwrap()).collect();

        mac
    }).collect();

    let mut full_configure_presses = 0;
    for m in machines {

        let n_buttons = m.button_sets.len();
        let mut fewest_presses = usize::MAX;

        for i in 0..(1u64 << n_buttons) {
            let mut state = 0u32;
            let mut count = 0;

            for j in 0..n_buttons {
                if (i & (1 << j)) != 0 {
                    state ^= m.button_sets[j];
                    count += 1;
                }
            }

            if state == m.lights {
                if count <= fewest_presses {
                    fewest_presses = count;
                }
            }

        }

        full_configure_presses += fewest_presses;

        //println!("Lights {:?}, Buttons {:?}, Joltage {:?}", m.lights, m.button_sets, m.joltage);
    }

    println!("Part 1 Answer = {}", full_configure_presses);
}

fn part2(_contents: &str)
{
    println!("Part 2 Answer = ");
}