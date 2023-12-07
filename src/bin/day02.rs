use std::fs;

struct GameData{
    game_id: i32,
    red_count: i32,
    blue_count: i32,
    green_count: i32,
}

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

fn main(){
    let contents = fs::read_to_string("./inputs/input2.txt").expect("Not Found");
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str){
    let mut games: Vec<GameData> = Vec::new();

    //let lines: Vec<&str> = contents.split('\n').collect();
    let lines = contents.lines();
    for line in lines{
        let mut game_data: GameData = GameData { game_id: 0, red_count: 0, blue_count: 0, green_count: 0 };

        let splits: Vec<&str> = line.split(&[':', ',', ';']).collect();
        for split in splits{
//            println!("{}", split);

            if split.contains("Game"){
                let s = split.strip_prefix("Game ").unwrap();
                game_data.game_id = s.parse::<i32>().unwrap();
            }
            if split.contains("red"){
                let d = split.strip_suffix(" red").unwrap();
                let s = d.replace(" ", "");
                let n =s.parse::<i32>().unwrap(); 
                if game_data.red_count < n
                {
                    game_data.red_count = n;
                }
            }
            if split.contains("green"){
                let d = split.strip_suffix(" green").unwrap();
                let s = d.replace(" ", "");
//                println!("{}", s);
                let n =s.parse::<i32>().unwrap(); 
                if game_data.green_count < n
                {
                    game_data.green_count = n;
                }
            }
            if split.contains("blue"){
                let d = split.strip_suffix(" blue").unwrap();
                let s = d.replace(" ", "");
                let n =s.parse::<i32>().unwrap(); 
                if game_data.blue_count < n
                {
                    game_data.blue_count = n;
                }
            }
        }

        games.push(game_data);
    }

    let mut sum_of_gameid: i32 = 0;
    for game in games{
        if game.red_count > RED_MAX || game.green_count > GREEN_MAX || game.blue_count > BLUE_MAX
        {
            continue;
        }
//        println!("Game {}, Red: {}, Green: {}, Blue: {}", game.game_id, game.red_count, game.green_count, game.blue_count);
        sum_of_gameid += game.game_id;
    }

    println!("Part 1 Answer = {}", sum_of_gameid.to_string());
}

fn part2(contents: &str){   
    let mut games: Vec<GameData> = Vec::new();

    //let lines: Vec<&str> = contents.split('\n').collect();
    let lines = contents.lines();
    for line in lines{
        let mut game_data: GameData = GameData { game_id: 0, red_count: 0, blue_count: 0, green_count: 0 };

        let splits: Vec<&str> = line.split(&[':', ',', ';']).collect();
        for split in splits{
//            println!("{}", split);

            if split.contains("Game"){
                let s = split.strip_prefix("Game ").unwrap();
                game_data.game_id = s.parse::<i32>().unwrap();
            }
            if split.contains("red"){
                let d = split.strip_suffix(" red").unwrap();
                let s = d.replace(" ", "");
                let n =s.parse::<i32>().unwrap(); 
                if game_data.red_count < n
                {
                    game_data.red_count = n;
                }
            }
            if split.contains("green"){
                let d = split.strip_suffix(" green").unwrap();
                let s = d.replace(" ", "");
//                println!("{}", s);
                let n =s.parse::<i32>().unwrap(); 
                if game_data.green_count < n
                {
                    game_data.green_count = n;
                }
            }
            if split.contains("blue"){
                let d = split.strip_suffix(" blue").unwrap();
                let s = d.replace(" ", "");
                let n =s.parse::<i32>().unwrap(); 
                if game_data.blue_count < n
                {
                    game_data.blue_count = n;
                }
            }
        }

        games.push(game_data);
    }

    let mut sum_of_power: i32 = 0;
    for game in games{
//        println!("Game {}, Red: {}, Green: {}, Blue: {}", game.game_id, game.red_count, game.green_count, game.blue_count);
        sum_of_power += game.red_count * game.green_count * game.blue_count;
    }

    println!("Part 2 Answer = {}", sum_of_power.to_string());
}
