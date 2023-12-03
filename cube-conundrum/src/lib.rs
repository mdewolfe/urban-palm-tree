/**
* Day 1, Part 2
* Cube Conundrum
*/
use std::str::FromStr;

use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
#[allow(dead_code)]
pub struct CubeResult {
    sum: u32,
    power_sum: u32,
}
pub fn go() -> CubeResult {
    let file = File::open("./cube-conundrum/cube-conundrum.txt").unwrap();

    let control: Game = Game::control_game();
    let mut sum: u32 = 0;
    let mut power_sum: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(game_input) = line {
            if game_input.len() == 0 {
                continue;
            }
            let other_game: Game = Game::from_string(game_input);
            if control.can_play(&other_game) {
                sum += other_game.id;
            }
            power_sum += other_game.power;
        }
    }
    return CubeResult { sum, power_sum };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Colour {
    RED,
    GREEN,
    BLUE,
}

impl Colour {
    fn from_string(input: &str) -> Colour {
        return match input {
            "red" => Colour::RED,
            "green" => Colour::GREEN,
            _ => Colour::BLUE,
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    colour: Colour,
    count: u32,
}

impl Round {
    fn from_lines(input: Vec<String>) -> Vec<Round> {
        let mut rounds: Vec<Round> = Vec::new();
        for line in input {
            for x in line.split(", ") {
                let comps = x.split(" ").collect::<Vec<&str>>();
                let count: u32 = FromStr::from_str(comps[0]).unwrap();

                rounds.push(Round {
                    count,
                    colour: Colour::from_string(comps[1]),
                });
            }
        }

        return rounds;
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
    power: u32,
}

impl Game {
    fn from_string(input: String) -> Game {
        let game_info: Vec<&str> = input.split(": ").collect::<Vec<&str>>();
        let game_id_info = game_info[0].split(" ").collect::<Vec<&str>>();
        let game_id: u32 = game_id_info[1].parse::<u32>().unwrap();

        let rounds: Vec<String> = game_info[1].split("; ").map(|s| s.to_string()).collect();
        let game_rounds: Vec<Round> = Round::from_lines(rounds);

        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for r in &game_rounds {
            match r.colour {
                Colour::RED => {
                    if r.count > min_red {
                        min_red = r.count;
                    }
                }
                Colour::BLUE => {
                    if r.count > min_blue {
                        min_blue = r.count;
                    }
                }
                Colour::GREEN => {
                    if r.count > min_green {
                        min_green = r.count;
                    }
                }
            }
        }

        return Game {
            id: game_id,
            rounds: game_rounds,
            power: min_red * min_green * min_blue,
        };
    }

    fn can_play(&self, other: &Game) -> bool {
        for other_round in &other.rounds {
            let self_round = self
                .rounds
                .clone()
                .into_iter()
                .find(|r| other_round.colour == r.colour)
                .unwrap();
            if self_round.count < other_round.count {
                return false;
            }
        }
        return true;
    }

    fn control_game() -> Game {
        return Game {
            id: 0,
            rounds: vec![
                Round {
                    colour: Colour::RED,
                    count: 12,
                },
                Round {
                    colour: Colour::GREEN,
                    count: 13,
                },
                Round {
                    colour: Colour::BLUE,
                    count: 14,
                },
            ],
            power: 0,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_game() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = Game::from_string(input.to_string());

        assert_eq!(game.id, 1);
        assert_eq!(game.rounds.len(), 6);
        assert_eq!(game.power, 48);

        assert_eq!(game.rounds[0].colour, Colour::BLUE);
        assert_eq!(game.rounds[0].count, 3);

        assert_eq!(game.rounds[1].colour, Colour::RED);
        assert_eq!(game.rounds[1].count, 4);

        assert_eq!(game.rounds[2].colour, Colour::RED);
        assert_eq!(game.rounds[2].count, 1);

        assert_eq!(game.rounds[3].colour, Colour::GREEN);
        assert_eq!(game.rounds[3].count, 2);

        assert_eq!(game.rounds[4].colour, Colour::BLUE);
        assert_eq!(game.rounds[4].count, 6);

        assert_eq!(game.rounds[5].colour, Colour::GREEN);
        assert_eq!(game.rounds[5].count, 2);
    }

    #[test]
    fn can_play_game_one() {
        let control: Game = Game::control_game();
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = Game::from_string(input.to_string());

        assert!(control.can_play(&game));
    }

    #[test]
    fn can_play_game_two() {
        let control: Game = Game::control_game();
        let input: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game: Game = Game::from_string(input.to_string());

        assert!(control.can_play(&game));
        assert_eq!(game.power, 12);
    }

    #[test]
    fn can_play_game_three() {
        let control: Game = Game::control_game();
        let input: &str =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game: Game = Game::from_string(input.to_string());

        assert!(!control.can_play(&game));
        assert_eq!(game.power, 1560);
    }

    fn sum_games(control: &Game, games: &Vec<Game>) -> (u32, u32) {
        let mut sum: u32 = 0;
        let mut power_sum: u32 = 0;
        for game in games {
            if control.can_play(&game) {
                sum += game.id;
            }
            power_sum += game.power;
        }

        return (sum, power_sum);
    }

    #[test]
    fn sum_game_ids() {
        let games: Vec<Game> = vec![
            Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game::from_string(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            ),
            Game::from_string(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
            ),
            Game::from_string(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
            ),
            Game::from_string("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()),
        ];

        let control: Game = Game::control_game();
        let (sum, power_sum): (u32, u32) = sum_games(&control, &games);
        assert_eq!(sum, 8);
        assert_eq!(power_sum, 2286);
    }
}
