use std::env;
use rand::Rng;

fn main() {
    let luck: i32 = rand::thread_rng().gen_range(1..101);

    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} requires four numbers", args[0]);
    }

    let mut Player_1: i32 = args[1].parse().expect("1. Player is not a valid number");
    let mut Player_2: i32 = args[2].parse().expect("2. Player is not a valid number");
    let mut Player_3: i32 = args[3].parse().expect("3. Player is not a valid number");
    let mut Player_4: i32 = args[4].parse().expect("4. Player is not a valid number");

    let mut player_1 = Player_1 - luck;
    let mut player_2 = Player_2 - luck;
    let mut player_3 = Player_3 - luck;
    let mut player_4 = Player_4 - luck;

    let mut Players = vec![player_1.abs(), player_2.abs(), player_3.abs(), player_4.abs()];
    let mut winner: i32 = 101;
    let mut winning: usize =0;

    println!("Lucky number is {}", luck);

    for x in 0..Players.len() {
        if Players[x] < winner {
            winner = Players[x];
            winning = x;
        }
    }
    match winning {
        0 => println!("Congratulations Player 1 winsss 🎉"),
        1 => println!("Congratulations Player 2 winsss 🎉"),
        2 => println!("Congratulations Player 3 winsss 🎉"),
        3 => println!("Congratulations Player 4 winsss 🎉"),
        _ => println!("No winner sorry"),
    }
    println!("Difference {}", winner);
}
