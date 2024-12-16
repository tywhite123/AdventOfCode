use std::fs;

use regex::Regex;

const A_PRICE: f64 = 3.0;
const B_PRICE: f64 = 1.0;

struct Machine {
    prize: (f64, f64),
    a_movement: (f64, f64),
    b_movement: (f64, f64)

}

fn main() {
    println!("Day 13:");
    
    let contents = fs::read_to_string("./inputs/input13.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let machine_strings: Vec<&str> = contents.split("\n\n").collect();

    let mut machines: Vec<Machine> = vec![];
    for m in machine_strings {

        let values: Vec<f64> = Regex::new(r"[0-9]+").unwrap()
            .find_iter(m)
            .filter_map(|f| Some(f.as_str().parse::<f64>().unwrap())).collect();

        let mac = Machine{prize: (values[4], values[5]), a_movement: (values[0], values[1]), b_movement: (values[2], values[3])};
        machines.push(mac);
    }

    let mut total: f64 = 0.0;
    for machine in machines {
        let total_a_press = (machine.prize.0 * machine.b_movement.1 - machine.prize.1 * machine.b_movement.0) 
            / (machine.a_movement.0 * machine.b_movement.1 - machine.a_movement.1 * machine.b_movement.0);
        let total_b_press = (machine.prize.0 * machine.a_movement.1 - machine.prize.1 * machine.a_movement.0) 
            / (machine.a_movement.0 * machine.b_movement.1 - machine.a_movement.1 * machine.b_movement.0);

        if total_a_press.fract().abs() > 0.0 || total_b_press.fract().abs() > 0.0 { continue; }

        total += (total_a_press.abs() * A_PRICE) + (total_b_press.abs() * B_PRICE);
    }

    
    println!("Part 1 Answer = {}", total);
}

fn part2(contents: &str)
{
    let machine_strings: Vec<&str> = contents.split("\n\n").collect();

    let mut machines: Vec<Machine> = vec![];
    for m in machine_strings {

        let values: Vec<f64> = Regex::new(r"[0-9]+").unwrap()
            .find_iter(m)
            .filter_map(|f| Some(f.as_str().parse::<f64>().unwrap())).collect();

        let mac = Machine{prize: (values[4]+10000000000000.0, values[5]+10000000000000.0), a_movement: (values[0], values[1]), b_movement: (values[2], values[3])};
        //println!("{} {:?}", 10000000000000.0+values[4], mac.prize);
        machines.push(mac);
    }

    let mut total: f64 = 0.0;
    for machine in machines {
        let total_a_press = (machine.prize.0 * machine.b_movement.1 - machine.prize.1 * machine.b_movement.0) 
            / (machine.a_movement.0 * machine.b_movement.1 - machine.a_movement.1 * machine.b_movement.0);
        let total_b_press = (machine.prize.0 * machine.a_movement.1 - machine.prize.1 * machine.a_movement.0) 
            / (machine.a_movement.0 * machine.b_movement.1 - machine.a_movement.1 * machine.b_movement.0);

        
        //println!("{:?}", machine.prize);
        if total_a_press.fract().abs() > 0.0 || total_b_press.fract().abs() > 0.0 { continue; }
        //println!("{}", total_a_press);

        total += (total_a_press.abs() * A_PRICE) + (total_b_press.abs() * B_PRICE);
    }

    println!("Part 2 Answer = {}", total);
}