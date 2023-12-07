
const INPUT: &str =
"Time:        56     71     79     99
Distance:   334   1135   1350   2430
";


fn main() {
    let races = parse(INPUT.lines().collect());
    let result: u64 = part_one(&races);
    println!("Day 6 Part 1: {:?}", result);

    let race: Race = parse_part_two(INPUT.lines().collect());
    let result: u64 = part_one(&vec![race]);
    println!("Day 6 Part 2: {:?}", result);
}

fn parse(lines: Vec<&str>) -> Vec<Race> {
    let ts: Vec<&str> = lines[0].split_whitespace().collect::<Vec<&str>>();
    let ds: Vec<&str> = lines[1].split_ascii_whitespace().collect();
    let mut bs: Vec<Race> = Vec::with_capacity(ts.len() - 1);
    for (t, d) in ts[1..].iter().zip(ds[1..].iter()) {
        bs.push(
            Race {
                time: t.parse::<u64>().expect("no beuno"),
                distance: d.parse::<u64>().expect("no beuno")
            }
        );
    }

    return bs;

}

fn parse_part_two(lines: Vec<&str>) -> Race {

    let ts: Vec<&str> = lines[0].split_whitespace().collect::<Vec<&str>>();
    let ds: Vec<&str> = lines[1].split_ascii_whitespace().collect();

    let time: u64 = ts[1..].join("").parse::<u64>().expect("no beuno");
    let distance: u64 = ds[1..].join("").parse::<u64>().expect("no beuno");

    return Race { time, distance };
}

fn part_one(races: &Vec<Race>) -> u64 {
    let mut result: u64 = 1;
    for race in races {
        let mut count: u64 = 0;
        for x in 1..race.time {
            let d: u64 = (race.time - x) * x;
            if d > race.distance {
                count += 1;
            }

        }

        result = result * count;
    }
    return result;
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
"Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_one() {
        let races = parse(TEST_INPUT.lines().collect());
        let result: u64 = part_one(&races);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_two() {
        let race = parse_part_two(TEST_INPUT.lines().collect());
        let result: u64 = part_one(&vec![race]);
        assert_eq!(result, 71503)
    }
}


