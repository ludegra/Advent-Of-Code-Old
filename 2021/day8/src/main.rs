use part1::part1;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/test.txt")
        .trim()
        .split('\n')
        .map(|s| {
            let mut split = s.split(" | ")
                .map(|s| {
                    s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()
                })
                .collect::<Vec<_>>();
            split[0].sort_by(|a, b| a.len().cmp(&b.len()));
            (split[0].clone(), split[1].clone())
        })
        .collect::<Vec<_>>();

    part1(&input);
}
