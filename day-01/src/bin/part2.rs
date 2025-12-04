fn main() {
    let data = include_str!("../../../input/day-01.txt");
    let answer = part2(data);
    println!("{:?}", answer);
}

fn part2(data: &str) -> i64 {
    let mut dial: i64 = 50;
    let mut zeros = 0;

    let mut dial_old = dial;
    for line in data.lines() {
        let (direction, steps) = line.split_at(1);
        let steps: i64 = steps.parse().unwrap();

        match direction {
            "L" => dial -= steps,
            _ => dial += steps,
        };

        // Round down to lower 0 and calculate how many times through 0.
        let rounded = (dial as f64 / 100.0).floor() as i64;
        let rounded_old = (dial_old as f64 / 100.0).floor() as i64;
        zeros += (rounded - rounded_old).abs();

        // Ending on 0 for a left move is not captured above.
        if direction == "L" && dial % 100 == 0 && dial_old % 100 != 0 {
            zeros += 1
        };

        // Starting on 0 and moving left is overstated.
        if direction == "L" && dial % 100 != 0 && dial_old % 100 == 0 {
            zeros -= 1
        };

        dial_old = dial;
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let data = include_str!("../../../input/day-01_test.txt");
        let answer = part2(data);
        println!("ok");
        assert_eq!(6, answer);
    }
}
