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
pub enum Game {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Lost,
    Draw,
}

pub fn play(a: Game, b: Game) -> GameResult {
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
}