fn part1(data: &str) -> usize {
    let mut lines = data.lines();
    let first_line = lines.next().unwrap().trim();
    let beam_start = (first_line.len() - 1) / 2;

    let splitter_rows: Vec<Vec<_>> = std::iter::once(vec![beam_start])
        .chain(lines.map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(ctr, ch)| (ch == '^').then_some(ctr))
                .collect()
        }))
        .collect();

    let mut current_beams = vec![beam_start];
    let mut num_splits = 0;

    for splitters in splitter_rows.iter().skip(1) {
        let mut next_beams = Vec::new();

        for &beam in &current_beams {
            if splitters.contains(&beam) {
                num_splits += 1;
                next_beams.push(beam - 1);
                next_beams.push(beam + 1);
            } else {
                next_beams.push(beam);
            }
        }

        next_beams.sort_unstable();
        next_beams.dedup();
        current_beams = next_beams;
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
