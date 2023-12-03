/**
* Day 3
* Gear Ratios
*/
use std::fs;

#[derive(Debug)]
pub struct GearRatio {
    pub value: u32,
}

pub fn go() -> GearRatio {
    let contents: String = fs::read_to_string("./gear_ratios/gear.ratio.input.txt").unwrap();
    let input: Vec<Vec<char>> = build_input(contents);
    let numbers: Vec<u32> =  process_parts(&input);
    return GearRatio { value: numbers.iter().sum() }
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


#[cfg(test)]
mod teststwo {
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
    fn test_ratios() {
        assert_eq!(5,4 );
    }
    #[test]
    fn calculate_all() {
        assert_eq!(0, 467835);
    }
}
