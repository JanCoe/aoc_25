fn part1(data: &str) -> u32 {
    data.lines()
        .filter_map(|record| {
            let first = record.chars().take(record.len() - 1).max()?;
            let after = record.find(first)? + 1;
            let second = record[after..].chars().max()?;
            Some(first.to_digit(10)? * 10 + second.to_digit(10)?)
        })
        .sum()
}

fn main() {
    let data = include_str!("../../../input/day-03.txt");
    println!("answer: {}", part1(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, part1(data));
    }
}
