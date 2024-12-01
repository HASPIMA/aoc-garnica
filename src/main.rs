mod days;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).expect("No day provided");
    let part = args.get(2).expect("No part provided");

    let res = match (day.as_str(), part.as_str()) {
        ("1", "1") => days::day1::part1("input1.txt"),
        ("1", "2") => days::day1::part2("input1.txt"),
        _ => unimplemented!(),
    };

    match res {
        Ok(result) => println!("Result day {} part {}: {}", day, part, result),
        Err(err) => eprintln!("Error day {} part {}: {}", day, part, err),
    }
}
