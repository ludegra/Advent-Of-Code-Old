use crate::structs::{Point, Basin, CouldExpand};

pub fn part2(input: Vec<Vec<u32>>, low_points: Vec<Point>) {
    let mut basin_sizes = Vec::new();

    for point in low_points {
        // println!("----------------------------------");
        let mut basin = Basin::new(point, &input); 
        // basin.print();

        let mut result = CouldExpand::Success;

        while let CouldExpand::Success = result {
            result = basin.increase_level(None);

            if let CouldExpand::Faliure(size) = result {
                basin_sizes.push(size);
            }
        }
    }
    basin_sizes.sort();
    basin_sizes = basin_sizes.iter().rev().map(|s| s.clone()).collect();
    println!("Part 2: {}", basin_sizes[0..3].iter().map(|s| s.clone()).product::<usize>());
}