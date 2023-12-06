/**
* Day 3
* Gear Ratios
*/
use std::fs;

#[derive(Debug)]
pub struct GearRatio {
    pub part_1: u32,
    pub part_2: u64,
}

pub fn go() -> GearRatio {
    let contents: String = fs::read_to_string("./gear_ratios/gear.ratio.input.txt").unwrap();
    let input: Vec<Vec<char>> = build_input(contents);
    let numbers: Vec<u32> =  process_parts(&input);

    let tokens: Vec<Token> = find_tokens(&input);
    let pairs: Vec<GearPair> = find_gear_pairs(&tokens);
    let ratio_sum: u64 = pairs.iter().map(|gp| gp.ratio() ).sum();
    return GearRatio { part_1: numbers.iter().sum(), part_2: ratio_sum }
}

fn build_input(input: String) -> Vec<Vec<char>> {
    return input.lines().map(|l| l.chars().collect()).collect();
}

fn get_parts(line: &Vec<char>, previous_line: &Vec<char>, next_line: &Vec<char>) -> Vec<u32> {

    let mut idx: usize = 0;
    let line_length: usize = line.len();


    let mut results: Vec<u32> = Vec::new();
    let mut tracked: Vec<char> = Vec::new();
    while idx < line_length {
        let mut working: String = String::new();
        if !line[idx].is_ascii_digit() {
            tracked.push(line[idx]);
            idx += 1;
            continue;
        }

        working.push(line[idx]);

        let mut inner_idx: usize = idx + 1;
        while inner_idx < line_length && line[inner_idx].is_ascii_digit() {
            working.push(line[inner_idx]);
            inner_idx += 1;
        }
        if idx > 0 && line[idx-1] != '.' {
            results.push(working.parse::<u32>().unwrap());
            idx = inner_idx;
            continue;
        }

        if inner_idx < line_length - 1 && line[inner_idx] != '.' {
            results.push(working.parse::<u32>().unwrap());
            idx = inner_idx;
            continue;
        }

        if inner_idx == line_length {
            idx = inner_idx;
            continue;
        }

        let mut search_idx: usize = if idx > 0 {
            idx - 1
        } else {
            idx
        };
        while search_idx <=inner_idx {
            if previous_line.len() > 0 && !previous_line[search_idx].is_ascii_digit() && previous_line[search_idx] != '.' {
                results.push(working.parse::<u32>().unwrap());
                break;
            }

            if next_line.len() > 0 && !next_line[search_idx].is_ascii_digit() && next_line[search_idx] != '.'  {
                results.push(working.parse::<u32>().unwrap());
                break;
            }

            search_idx += 1;
        }

        idx = inner_idx;
    }

    return results;
}

fn process_parts(input: &Vec<Vec<char>> ) -> Vec<u32> {
    let empty: Vec<char> = Vec::with_capacity(0);
    let max_idx: usize = input.len() - 1;
    let mut part_numbers: Vec<u32> = Vec::new();
    for (idx, line) in input.iter().enumerate() {
        let previous_line: &Vec<char> = if idx == 0 {
            &empty
        } else {
            &input[idx-1]
        };

        let next_line: &Vec<char> = if idx == max_idx {
            &empty
        } else {
            &input[idx + 1]
        };

        let mut parts: Vec<u32> = get_parts(&line, previous_line, next_line);
        if parts.len() > 0 {
            part_numbers.append(&mut parts);
        }
    }

    return part_numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn parse_input() {
        let s: String = String::from(RAW);
        let result: Vec<Vec<char>> = build_input(s);

        let expected: Vec<Vec<char>> = RAW.lines().map(|l| l.chars().collect() ).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn get_numbers() {
        let s: String = String::from(RAW);
        let input: Vec<Vec<char>> = build_input(s);
        let numbers: Vec<u32> = process_parts(&input);
        assert_eq!(numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn test_sum() {
        let input: Vec<Vec<char>> = build_input(String::from(RAW));
        let numbers: Vec<u32> = process_parts(&input);
        assert_eq!(numbers.iter().sum::<u32>(), 4361);
    }
}


#[derive(Debug)]
struct GearPair {
    value_1: Token,
    value_2: Token,
}

impl GearPair {
    fn ratio(&self) -> u64 {
        return self.value_1.value as u64 * self.value_2.value as u64;
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Number,
    Gear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    value: u32,
    x: i32,
    xx: i32,
    y: i32,
    token_type: Type,
}

impl Token {
    fn collides(&self, other: &Token) -> bool {
        if other.y < self.y - 1 || other.y > self.y + 1 {
            return false;
        }

        if other.xx < self.x - 1 || other.x > self.xx + 1 {
            return false;
        }

        return true;
    }
}

fn find_tokens(lines: &Vec<Vec<char>>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut digits: String = String::new();
    for (row, line) in lines.iter().enumerate() {
        for (idx, ch) in line.iter().enumerate() {
            if *ch == '*' {
                tokens.push(Token { value: 0, x: idx as i32, y: row as i32, xx: idx as i32, token_type: Type::Gear });
                continue;
            }

            if ch.is_ascii_digit() {
                digits.push(*ch);
                continue;
            } else if digits.len() > 0 {
                let x: i32 = if idx == 0 {
                    idx as i32
                } else {
                    (idx - digits.len()) as i32
                };
                let xx: i32 = if idx == 0 {
                    idx as i32
                } else {
                    (idx - 1) as i32
                };
                tokens.push(Token { value: digits.parse::<u32>().unwrap(), x, xx, y: row as i32, token_type: Type::Number });
                digits = String::new();

            }
        }
    }

    return tokens;
}

fn find_gear_pairs(tokens: &Vec<Token>) -> Vec<GearPair> {
    let gears: Vec<&Token> = tokens.iter().filter(|g| g.token_type == Type::Gear).collect();
    let numbers: Vec<&Token> = tokens.iter().filter(|n| n.token_type == Type::Number).collect();

    let mut pairs: Vec<GearPair> = Vec::new();
    for gear in gears {
        let mut temp: Vec<&Token> = Vec::new();
        for n in &numbers {
            if gear.collides(n) {
                temp.push(n);
            }
        }

        if temp.len() >= 2 {
            pairs.push(GearPair{ value_1: temp[0].clone(), value_2: temp[1].clone() });
        }
    }

    return pairs;
}


#[cfg(test)]
mod teststwo {
    use super::*;

    #[test]
    fn test_find_gears() {
        let raw: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let lines: Vec<Vec<char>> = build_input(raw.to_string());
        let gears: Vec<Token> = find_tokens(&lines);
        assert_eq!(gears, vec![
            Token {value: 0, y: 1, x: 0, xx: 3, token_type: Type::Gear },
            Token {value: 0, y: 4, x: 3, xx: 3, token_type: Type::Gear },
            Token {value: 0, y: 8, x: 5, xx: 5, token_type: Type::Gear },
        ]);
    }


    #[test]
    fn test_find_numbers() {

        let raw: &str =
"467..114..
...*......
..35..633.
......#...
";
        let lines: Vec<Vec<char>> = build_input(raw.to_string());
        let  numbers: Vec<Token> = find_tokens(&lines).into_iter().filter(|t| t.token_type == Type::Number ).collect();
        assert_eq!(numbers, vec![
            Token {value: 467, y: 0, x: 0, xx: 2, token_type: Type::Number },
            Token {value: 114, y: 0, x: 5, xx: 7, token_type: Type::Number },
            Token {value: 35, y: 2, x: 2, xx: 3, token_type: Type::Number },
            Token {value: 633, y: 2, x: 6, xx: 8, token_type: Type::Number }
        ]);
    }

    #[test]
    fn test_ratio() {
        let raw: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let lines: Vec<Vec<char>> = build_input(raw.to_string());
        let tokens: Vec<Token> = find_tokens(&lines);
        let pairs: Vec<GearPair> = find_gear_pairs(&tokens);
        let ratio_sum: u64 = pairs.iter().map(|gp| gp.ratio() ).sum();
        assert_eq!(ratio_sum, 467835);

    }


}
