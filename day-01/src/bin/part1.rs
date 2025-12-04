fn main() {
    let data = include_str!("../../../input/day-01.txt");
    let answer = part1(data);
    println!("{:?}", answer);
}

fn part1(data: &str) -> i64 {
    let mut dial: i64 = 50;
    let mut zeros = 0;

    for line in data.lines() {
        let (direction, steps) = line.split_at(1);
        let steps: i64 = steps.parse().unwrap();

        match direction {
            "L" => dial -= steps,
            _ => dial += steps,
        };

        dial = if dial < 0 { (100 + dial % 100) % 100 } else { dial % 100 };

        if dial == 0 {
            zeros += 1
        }
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = include_str!("../../../input/day-01_test.txt");
        let answer = part1(data);
        assert_eq!(3, answer);
    }
}
