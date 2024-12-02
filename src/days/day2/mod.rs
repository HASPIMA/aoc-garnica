pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut safe = 0;
    for line in input.lines() {
        let res = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|w| w[0] - w[1])
            .try_fold("-", |acc, x| match (acc, x) {
                ("-", 1..=3) => Ok("asc"),
                ("-", -3..=-1) => Ok("desc"),
                ("asc", 1..=3) => Ok("asc"),
                ("desc", -3..=-1) => Ok("desc"),
                _ => Err(()),
            });
        if res.is_ok() {
            safe += 1;
        }
    }
    Ok(safe)
}

pub fn part2(_input: &str) -> Result<i32, &'static str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"7 6 4 2 1
                       1 2 7 8 9
                       9 7 6 2 1
                       1 3 2 4 5
                       8 6 4 4 1
                       1 3 6 7 9"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        unimplemented!();
    }
}
