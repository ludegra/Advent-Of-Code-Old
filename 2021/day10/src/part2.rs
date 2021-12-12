pub fn part2(legal_lines: Vec<String>) {
    let mut scores = Vec::new();

    for line in legal_lines {
        let mut score: u64 = 0;

        for opening_bracket in line.chars().rev() {
            let bracket_score = match opening_bracket {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };

            score *= 5;
            score += bracket_score;
        }

        scores.push(score);
    }

    scores.sort();
    let len = scores.len();
    println!("Part 2: {}", scores[len/2]);
}