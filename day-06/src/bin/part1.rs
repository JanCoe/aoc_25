fn part1(data: &str) -> u64 {
    let mut lines: Vec<_> = data.lines().collect();
    let operator = lines.pop().expect("missing operators");

    let grid1: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|chars| chars.parse().expect("invalid number"))
                .collect()
        })
        .collect();

    let grid2: Vec<Vec<_>> = (0..grid1[0].len())
        .map(|i| grid1.iter().map(|row| row[i]).collect())
        .collect();

    operator
        .split_whitespace()
        .enumerate()
        .map(|(ctr, op)| match op {
            "*" => grid2[ctr].iter().product::<u64>(),
            "+" => grid2[ctr].iter().sum(),
            _ => panic!("unknown operator: {op}"),
        })
        .sum()
}

fn main() {
    let data = include_str!("../../../input/day-06.txt");
    // let data = "123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +";
    println!("{}", part1(data));
}
