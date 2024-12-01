use std::fs::read_to_string;

mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).expect("No day provided");
    let part = args.get(2).expect("No part provided");

    let res = match (day.as_str(), part.as_str()) {
        ("1", "1") => days::day1::part1(&read_to_string("data/days/day1/input1.txt").unwrap()),
        ("1", "2") => days::day1::part2(&read_to_string("data/days/day1/input1.txt").unwrap()),
        _ => unimplemented!(),
    };

    match res {
        Ok(result) => println!("Result day {} part {}: {}", day, part, result),
        Err(err) => eprintln!("Error day {} part {}: {}", day, part, err),
    }
}
