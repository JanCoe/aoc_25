fn part1(data: &str) -> usize {
    let mut splitters: Vec<_> = Vec::new();
    let line_length = data.lines().next().unwrap().trim().len();
    let beam_start = (line_length - 1) / 2;

    // Each row represents positions of splitters
    splitters.push(vec![beam_start]);
    data.lines()
        .skip(1)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(ctr, i)| if let '^' = i { Some(ctr) } else { None })
                .collect::<Vec<_>>()
        })
        .for_each(|indices| splitters.push(indices));

    let mut old_beams = vec![beam_start];
    let mut num_splits = 0;

    for row in splitters.into_iter().skip(1) {
        let mut new_beams = Vec::new();

        for &beam in &old_beams {
            if row.contains(&beam) {
                num_splits += 1;
                new_beams.push(beam - 1);
                new_beams.push(beam + 1);
            } else {
                new_beams.push(beam);
            }
        }

        new_beams.sort_unstable();
        new_beams.dedup();

        old_beams = new_beams;
    }
    num_splits
}

fn main() {
    let data = include_str!("../../../input/day-07.txt");
    //     let data = ".......S.......
    // ...............
    // .......^.......
    // ...............
    // ......^.^......
    // ...............
    // .....^.^.^.....
    // ...............
    // ....^.^...^....
    // ...............
    // ...^.^...^.^...
    // ...............
    // ..^...^.....^..
    // ...............
    // .^.^.^.^.^...^.
    // ...............";
    println!("{}", part1(data));
}
