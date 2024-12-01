use crate::utils::file::read_lines;

pub fn part1(filename: &str) -> Result<i32, &'static str> {
    let input_path = format!("data/days/day1/{}", filename);
    let lines = read_lines(input_path).map_err(|_| "Error reading file")?;

    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in lines.map_while(Result::ok) {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let distance = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| left - right)
        .map(|x| x.abs())
        .sum();

    Ok(distance)
}

pub fn part2(filename: &str) -> Result<i32, &'static str> {
    let input_path = format!("data/days/day1/{}", filename);
    let lines = read_lines(input_path).map_err(|_| "Error reading file")?;

    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in lines.map_while(Result::ok) {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    let similarity = left_list
        .iter()
        .map(|left_item| {
            let right_count = right_list.iter().filter(|x| *x == left_item).count();
            right_count as i32 * left_item
        })
        .sum();

    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = part1("example1.txt").unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let result = part2("example1.txt").unwrap();
        assert_eq!(result, 31);
    }
}
