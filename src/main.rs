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

fn main() {
    let a = Game::Paper;
    let b = Game::Scissor;
    println!("{:#?}", play(a, b));
    // define your games a and b
    // call play function with arguments
    // display result
}
