mod days;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).expect("No day provided");

    match day.as_str() {
        "1" => {
            days::day1::part1("input1.txt")
                .map(|x| println!("Res: {}", x))
                .unwrap();
        }
        _ => unimplemented!(),
    }
}
