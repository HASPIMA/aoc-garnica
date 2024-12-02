mod days;

fn main() {
    dotenv::dotenv().ok();
    let session = std::env::var("ADVENT_OF_CODE_SESSION").expect("No session provided");

    let args: Vec<String> = std::env::args().collect();
    let year = 2024;
    let day = args.get(1).expect("No day provided");
    let part = args.get(2).expect("No part provided");

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::new();
    let input = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response");

    let res = match (day.as_str(), part.as_str()) {
        ("1", "1") => days::day1::part1(&input),
        ("1", "2") => days::day1::part2(&input),
        ("2", "1") => days::day2::part1(&input),
        ("2", "2") => days::day2::part2(&input),
        _ => unimplemented!(),
    };

    match res {
        Ok(result) => println!("Result day {} part {}: {}", day, part, result),
        Err(err) => eprintln!("Error day {} part {}: {}", day, part, err),
    }
}
