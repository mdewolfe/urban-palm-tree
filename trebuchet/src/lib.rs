/**
* Day 1, Part 1
* Trebuchet
*/
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Trebuchet {
    sum: u32,
}

pub fn go() -> Trebuchet {
    let file = File::open("./trebuchet/trebuchet.input.txt").unwrap();

    let mut sum: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(calibration) = line {
            let value: u32 = find_calibration(calibration);
            sum += value;
        }
    }

    return Trebuchet { sum };
}

fn find_calibration(calibration: String) -> u32 {
    let filtered: String = calibration.chars().filter(|c| c.is_digit(10)).collect();
    if filtered.len() == 0 {
        return 0;
    }

    let as_chars: Vec<char> = filtered.chars().collect();
    let x: String = [as_chars[0], as_chars[as_chars.len() - 1]].iter().collect();
    match x.parse::<u32>() {
        Ok(value) => return value,
        _ => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_caligration_one() {
        let s: String = String::from("1abc2");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 12);
    }

    #[test]
    fn find_calibration_two() {
        let s: String = String::from("pqr3stu8vwx");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 38);
    }

    #[test]
    fn find_calibration_three() {
        let s: String = String::from("a1b2c3d4e5f");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 15);
    }

    #[test]
    fn find_calibration_four() {
        let s: String = String::from("treb7uchet");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 77);
    }

    #[test]
    fn find_calibration_zero_length() {
        let s: String = String::from("");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn find_calibration_five() {
        let s: String = String::from("2513");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 23);
    }

    #[test]
    fn find_calibration_six() {
        let s: String = String::from("a5sd7fgh5jkl");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 55);
    }

    #[test]
    fn find_calibration_seven() {
        let s: String = String::from("2eight3rcjheight67bmktvnqltv");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 27);
    }

    #[test]
    fn find_calibration_eight() {
        let s: String = String::from("four4four");
        let result: u32 = find_calibration(s);
        assert_eq!(result, 44);
    }

    #[test]
    fn find_sum() {
        let calibrations: Vec<String> = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        let mut sum: u32 = 0;

        for c in calibrations {
            let x: u32 = find_calibration(c);
            sum += x;
        }

        assert_eq!(sum, 142);
    }
}
