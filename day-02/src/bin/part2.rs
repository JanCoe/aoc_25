#![feature(iter_array_chunks)]

use itertools::Itertools;

fn main() {
    // let data = include_str!("../../../input/day-02.txt");
    let data = "998-1012";
    let answer = part2(data);
    println!("{:?}", answer);
}

fn part2(data: &str) -> i64 {
    let mut total: i64 = 0;
    for record in data.split(",") {
        let record = record.trim();
        if let Some((first, last)) = record.split_once("-") {
            let first: i64 = first.parse().unwrap();
            let last: i64 = last.parse().unwrap();

            for num in first..=last {
                let num_str = num.to_string();

                for chunk_size in 1..num_str.len() / 2 {
                    if num_str.chars().chunks(chunk_size).into_iter().all_equal() {
                        println!("{num:?}, 1");
                        total += num;
                    }
                }
                // else if num_str.chars().array_chunks::<2>().into_iter().all_equal() {
                //     println!("{num:?}, 2");
                //     total += num;
                // } else if num_str.chars().array_chunks::<3>().into_iter().all_equal() {
                //     println!("{num:?}, 3");
                //     total += num;
                // } else if num_str.chars().array_chunks::<4>().into_iter().all_equal() {
                //     println!("{num:?}, 4");
                //     total += num;
                // } else if num_str.chars().array_chunks::<5>().into_iter().all_equal() {
                //     println!("{num:?}, 5");
                //     total += num;
                // } else if num_str.chars().array_chunks::<6>().into_iter().all_equal() {
                //     println!("{num:?}, 6");
                //     total += num;
                // } else if num_str.chars().array_chunks::<7>().into_iter().all_equal() {
                //     println!("{num:?}, 7");
                //     total += num;
                // }
            }
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = part2(data);
        assert_eq!(1227775554, answer);
    }
}
