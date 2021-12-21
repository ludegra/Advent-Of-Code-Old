use std::collections::HashMap;

fn main() {
    let input = include_str!("../assets/input.txt").trim()[("target area: x=".len())..]
        .split(", y=")
        .map(|s| {
            let minmax = s
                .split("..")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (minmax[0], minmax[1])
        })
        .collect::<Vec<_>>();

    let ((x_min, x_max), (y_min, y_max)) = (input[0], input[1]);

    let mut possible_steps = Vec::new();
    let mut min_for_zero = i32::MAX;

    let mut vx_steps = HashMap::new();
    let mut vx_to_zero = HashMap::new();

    for vx in 0..=x_max {
        let (points, reached_zero) = closest_value_x(vx, y_min, x_max);
        for (closest, steps) in points {
            if closest >= x_min {
                if reached_zero {
                    if steps < min_for_zero {
                        min_for_zero = steps
                    }
                    let entry = vx_to_zero.entry(steps).or_insert(Vec::new());
                    entry.push(vx);
                }
                if !possible_steps.contains(&steps) {
                    possible_steps.push(steps);
                }
                let entry = vx_steps.entry(steps).or_insert(Vec::new());
                entry.push(vx);
            }
        }
    }

    let mut highest_highest = (0, 0);

    let mut pairs = Vec::new();

    for vy in -500..500 {
        let (points, highest) = closest_value_y(vy, y_max, y_min);

        for (closest, steps) in points {
            if closest <= y_max {
                if min_for_zero <= steps {
                    if highest_highest.0 < highest {
                        highest_highest = (highest, vy);
                    }

                    for key in vx_to_zero.keys() {
                        if *key <= steps {
                            for vx in vx_to_zero.get(key).unwrap() {
                                if !pairs.contains(&(*vx, vy)) {
                                    pairs.push((*vx, vy));
                                }
                            }
                        }
                    }
                }
                if possible_steps.contains(&steps) {
                    if highest_highest.0 < highest {
                        highest_highest = (highest, vy);
                    }

                    for vx in vx_steps.get(&steps).unwrap() {
                        if !pairs.contains(&(*vx, vy)) {
                            pairs.push((*vx, vy));
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", highest_highest.0);
    println!("Part 2: {}", pairs.len());
}

fn closest_value_x(v0: i32, min: i32, max: i32) -> (Vec<(i32, i32)>, bool) {
    let mut point = 0;
    let mut v = v0;
    let mut out_points = Vec::new();

    let decrementer = if v0 > 0 { 1 } else { -1 };

    let mut step_counter = 0;
    while point + v <= max && v != 0 {
        step_counter += 1;
        point += v;
        v -= decrementer;

        if point >= min {
            out_points.push((point, step_counter));
        }
    }
    (out_points, v == 0)
}
fn closest_value_y(v0: i32, max: i32, min: i32) -> (Vec<(i32, i32)>, i32) {
    let mut point = 0;
    let mut v = v0;
    let mut out_points = Vec::new();

    let mut step_counter = 0;
    let mut highest = 0;

    while point + v >= min {
        step_counter += 1;
        point += v;

        if v == 0 {
            highest = point;
        }
        if point <= max {
            out_points.push((point, step_counter));
        }

        v -= 1;
    }
    (out_points, highest)
}
