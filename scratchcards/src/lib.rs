/**
* Day 4, scratchcards
*/
use std::collections::HashSet;
use std::fs;


#[derive(Debug)]
pub struct ScratchResult {
    points: u32,
    copies: u32,
}

pub fn go() -> ScratchResult {
    let contents: String = fs::read_to_string("./scratchcards/scratchcards.input.txt").unwrap();
    let lines: Vec<&str> = contents.as_str().lines().collect();
    let games: Vec<Game> = build_games(&lines);
    let sum_points: u32 = games.iter().map(|g| g.points()).sum();
    return ScratchResult { points: sum_points, copies: find_copies(&games)  };
}

#[derive(Debug)]
struct Game {
    mine: HashSet<u32>,
    winning: HashSet<u32>,
}

impl Game {
    fn points(&self) -> u32 {
        let c: u32 = self.win_count();
        if c == 0 {
            return 0;
        }

        return 2_u32.pow(c - 1);
    }

    fn win_count(&self) -> u32 {
        return self
            .mine
            .intersection(&self.winning)
            .collect::<Vec<&u32>>()
            .len() as u32;
    }
}

fn find_copies(games: &Vec<Game>) -> u32 {
    let length: usize = games.len();

    let mut played: Vec<u32> = vec![0; length];
    for i in 0..length {
        played[i] += 1;
        let win_count: usize = games[i].win_count() as usize;
        let mut w: usize = i + 1;
        while w <= i+win_count {
            played[w] += played[i];
            w += 1;
        }
    }

    return played.iter().sum();
}

fn build_games(lines: &Vec<&str>) -> Vec<Game>{
    let mut games: Vec<Game> = Vec::with_capacity(lines.len());
    for line in lines {
        let s: Vec<&str> = line.split(": ").collect();
        let a:Vec<&str> = s[1].split(" | ").collect();

        let mut winning: HashSet<u32> = HashSet::new();
        for n in a[0].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap() ) {
            winning.insert(n);
        }

        let mut mine: HashSet<u32> = HashSet::new();
        for w in a[1].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap() ) {
            mine.insert(w);
        }

        games.push(Game { mine, winning });
    }

    return games;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    #[test]
    fn test_part_one() {
        let games: Vec<Game> = build_games(&INPUT.lines().collect());
        let result: u32 = games.iter().map(|g| g.points()).sum();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let games: Vec<Game> = build_games(&INPUT.lines().collect());
        let count: u32 = find_copies(&games);
        assert_eq!(count, 30);
    }

}
