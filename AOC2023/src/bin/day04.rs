use std::fs;

fn main()
{
    println!("Day 4:");  
    
    //let contents = fs::read_to_string("./inputs/test4.txt").expect("Not Found");
    let contents = fs::read_to_string("./inputs/intput4.txt").expect("Not Found");
    
    println!("{}", contents);

    part1(&contents);
    part2(&contents);

}

fn part1(contents: &str)
{
    let cards = contents.lines();
    let mut final_total = 0;

    for card in cards
    {
        let mut split_up = card.split([':','|']);
        let card_num = split_up.next().unwrap();
        let winning_nums: Vec<i32> = split_up.next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let nums: Vec<i32> = split_up.next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|n| winning_nums.contains(n))
            .collect();


        println!("{}", card_num);

        let mut card_total = 0;
        for _n in nums
        {
            if card_total < 1 {
                card_total += 1
            }else{
                card_total *= 2;
            }
        }

        final_total += card_total;
        println!("Total for card: {card_total}");
    }

    println!("Part 1 Answer = {final_total}");
}

fn part2(contents: &str)
{

    let cards = contents.lines();
    let size = contents.lines().count();
    //println!("{size}");
    let mut total_each_scratch = vec![1; size];
    let mut final_total = 0;

    let mut current_card_no = 0;
    for card in cards
    {
        let mut split_up = card.split([':','|']);
        let _card_num = split_up.next().unwrap();
        let winning_nums: Vec<i32> = split_up.next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let nums: Vec<i32> = split_up.next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|n| winning_nums.contains(n))
            .collect();


        //println!("{}", card_num);
        for n in 0..nums.len()
        {
            let current_index = current_card_no+n+1;
            //println!("{current_index}");
            if current_index < total_each_scratch.len()
            {
                println!("{} {}", current_card_no, current_index);
                total_each_scratch[current_index] += total_each_scratch[current_card_no];
            }
        }
        //println!("Total for card: {card_total}");
        //
        current_card_no += 1;
    }

    for card in total_each_scratch
    {
        final_total += card;
    }

    println!("Part 2 Answer = {final_total}");


}
