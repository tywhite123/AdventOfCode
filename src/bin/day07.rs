use std::fs;

struct Hand
{
    cards: String,
    bid: u32,
}

fn main() {
    println!("Day 07:");
    
    let contents = fs::read_to_string("./inputs/input7.txt").expect("Not Found");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str)
{
    let lines = contents.lines();
    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let hand_types: [(usize, usize); 7] = [(0,0), (2,0), (2,2), (3,0), (3,2), (4,0), (5,0)];
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines
    {
        let mut hand: Hand = Hand { cards: "".to_string(), bid: 0 };
        let splits: Vec<&str> = line
            .split_whitespace()
            .collect();
        
        hand.cards = splits[0].to_string();
        hand.bid = splits[1].parse::<u32>().unwrap();

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        let a_cards = &a.cards;
        let b_cards = &b.cards;

        let mut a_vec = vec![0; cards.len()];
        let mut b_vec = vec![0; cards.len()];

        for i in 0..cards.len()
        {
            a_vec[i] = a_cards.matches(cards[i]).count();
            b_vec[i] = b_cards.matches(cards[i]).count();
        }

        let mut a_matches = (0, 0);
        for r in a_vec
        {
            if r > 1
            {
                if r >= a_matches.0
                {
                    a_matches.1 = a_matches.0;
                    a_matches.0 = r;
                }
                else {
                    a_matches.1 = r;
                }
            }
        }
        
        let mut a_rank = hand_types.iter().position(|x| *x == a_matches).unwrap();
        //println!("{} {} {} {a_rank}", a_cards, a_matches.0, a_matches.1);

        let mut b_matches = (0, 0);
        for r in b_vec
        {
            if r > 1
            {
                if r >= b_matches.0
                {
                    b_matches.1 = b_matches.0;
                    b_matches.0 = r;
                }
                else {
                    b_matches.1 = r;
                }
            }
        }

        
        let mut b_rank = hand_types.iter().position(|x| *x == b_matches).unwrap();        
        //println!("{} {} {} {b_rank}", b_cards, b_matches.0, b_matches.1);

        //println!("{} {a_rank}, {} {b_rank}", a_cards, b_cards);

        if a_rank == b_rank
        {
            for i in 0..5
            {
                let a = a_cards.chars().nth(i).unwrap();
                let b = b_cards.chars().nth(i).unwrap();

                let c = cards.iter().position(|j| a == *j).unwrap();
                let c2 = cards.iter().position(|j| b == *j).unwrap();

                a_rank += if c > c2 {1} else {0};
                b_rank += if c < c2 {1} else {0};

                if a_rank > b_rank || b_rank > a_rank
                {
                    break;
                }
            }
        }
        

        a_rank.cmp(&b_rank)
    });
    
    let mut total: u32 = 0;
    for i in 0..hands.len()
    {
        //println!("{}", hands[i].cards);

        let s: String = format!("{}", i);
        total += hands[i].bid * (s.parse::<u32>().unwrap()+1);
    }

    //let x = splits[0].matches("3").count();

    println!("Part 1 Answer = {total}");
}

fn part2(contents: &str)
{
    let lines = contents.lines();
    let cards = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    let hand_types: [(usize, usize); 7] = [(1,1), (2,1), (2,2), (3,1), (3,2), (4,1), (5,0)];
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines
    {
        let mut hand: Hand = Hand { cards: "".to_string(), bid: 0 };
        let splits: Vec<&str> = line
            .split_whitespace()
            .collect();
        
        hand.cards = splits[0].to_string();
        hand.bid = splits[1].parse::<u32>().unwrap();

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        let a_cards = &a.cards;
        let b_cards = &b.cards;

        let mut a_vec = vec![0; cards.len()];
        let mut b_vec = vec![0; cards.len()];

        for i in 0..cards.len()
        {
            a_vec[i] = a_cards.matches(cards[i]).count();
            b_vec[i] = b_cards.matches(cards[i]).count();
        }

        let mut j_found = 0;
        let mut a_matches = (0, 0);
        for i in 0..a_vec.len()
        {
            let r = a_vec[i];
            if i == 0 && r < 5
            {
                j_found = r;
            }
            else if r > 0
            {
                if r >= a_matches.0
                {
                    a_matches.1 = a_matches.0;
                    a_matches.0 = r;
                }
                else if r >= a_matches.1{
                    a_matches.1 = r;
                }
            }  
            //println!("{} {} {}", a_cards, a_matches.0, a_matches.1);
        }
        a_matches.0 += j_found;
        
        //println!("{} {} {}", a_cards, a_matches.0, a_matches.1);
        let mut a_rank = hand_types.iter().position(|x| *x == a_matches).unwrap();

      
        j_found = 0;
        let mut b_matches = (0, 0);
        for i in 0..b_vec.len()
        {
            let r = b_vec[i];
            if i == 0 && r < 5
            {
                j_found = r;
            }
            else if r > 0
            {
                if r >= b_matches.0
                {
                    b_matches.1 = b_matches.0;
                    b_matches.0 = r;
                }
                else if r >= b_matches.1 {
                    b_matches.1 = r;
                }
            }
        }
        b_matches.0 += j_found;
        
        let mut b_rank = hand_types.iter().position(|x| *x == b_matches).unwrap();        
        //println!("{} {} {} {b_rank}", b_cards, b_matches.0, b_matches.1);

        //println!("{} {a_rank}, {} {b_rank}", a_cards, b_cards);

        if a_rank == b_rank
        {
            for i in 0..5
            {
                let a = a_cards.chars().nth(i).unwrap();
                let b = b_cards.chars().nth(i).unwrap();

                let c = cards.iter().position(|j| a == *j).unwrap();
                let c2 = cards.iter().position(|j| b == *j).unwrap();

                a_rank += if c > c2 {1} else {0};
                b_rank += if c < c2 {1} else {0};

                if a_rank > b_rank || b_rank > a_rank
                {
                    break;
                }
            }
        }
        

        a_rank.cmp(&b_rank)
    });
    
    let mut total: u32 = 0;
    for i in 0..hands.len()
    {
        //println!("{}", hands[i].cards);

        let s: String = format!("{}", i);
        total += hands[i].bid * (s.parse::<u32>().unwrap()+1);
    }
    println!("Part 2 Answer = {total}");
}