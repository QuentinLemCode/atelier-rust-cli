use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).expect("no argument");
    if cmd == "greets" {
        greets(args.get(2).expect("bad args"));
    }
    if cmd == "chifoumi" {
        let a = Game::try_from(args.get(2).expect("bad args")).expect("toto");
        let b = Game::try_from(args.get(3).expect("bad args")).expect("toto");
        println!("{:#?}", play(a, b));
    }
}

fn greets(name: &str) {
    println!("Hello, {} !", name);
}


impl TryFrom<&String> for Game {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value == "rock" {
            return Ok(Game::Rock);
        }
        if value == "paper" {
            return Ok(Game::Paper);
        }
        if value == "scissor" {
            return Ok(Game::Scissor);
        }
        Err("bad value")
    }
}

#[derive(Debug, PartialEq)]
enum Game {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Lost,
    Draw,
}

fn play(a: Game, b: Game) -> GameResult {
    if a == b {
        return GameResult::Draw;
    }
    if a == Game::Rock && b == Game::Scissor
        || a == Game::Paper && b == Game::Rock
        || a == Game::Scissor && b == Game::Rock
    {
        return GameResult::Win;
    }
    return GameResult::Lost;
    // your code
}

// fn main() {
//     let a = Game::Paper;
//     let b = Game::Scissor;
//     println!("{:#?}", play(a, b));
//     // define your games a and b
//     // call play function with arguments
//     // display result
// }
