fn main() {
    let data = include_str!("../../../input/day-02.txt");
    let answer = part1(data);
    println!("{:?}", answer);
}

fn part1(data: &str) -> i64 {
    let mut total: i64 = 0;
    for record in data.split(",") {
        let record = record.trim();
        if let Some((first, last)) = record.split_once("-") {
            let first: i64 = first.parse().unwrap();
            let last: i64 = last.parse().unwrap();

            for num in first..=last {
                let num_str = num.to_string();
                if num_str.len() % 2 == 0 {
                    let (one, two) = num_str.split_at(num_str.len() / 2);
                    if one == two {
                        total += num;
                    };
                }
            }
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = part1(data);
        assert_eq!(1227775554, answer);
    }
}
