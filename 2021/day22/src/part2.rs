pub fn part2(input: &Vec<(bool, (isize, isize), (isize, isize), (isize, isize))>) {
    let mut cubes = Vec::new();

    for (active, x, y, z) in input {
        if *active {
            add_cube(&mut cubes, (*active, *x, *y, *z))
        } else {
        }
    }
    println!("{:?}", cubes);
}

fn add_cube(
    cubes: &mut Vec<((isize, isize), (isize, isize), (isize, isize))>,
    cube: (bool, (isize, isize), (isize, isize), (isize, isize)),
) {
    let (active, x, y, z) = cube;

    let matching_x = find_in_dimension(cubes, (x, y, z), Dimension::X);
    let matching_y = find_in_dimension(&matching_x, (x, y, z), Dimension::Y);
    let matching = find_in_dimension(&matching_y, (x, y, z), Dimension::Z);

    if matching.is_empty() && active {
        cubes.push((x, y, z))
    } else {
        println!("\nCube: x: {}..{}, y: {}..{}, z: {}..{}", x.0, x.1, y.0, y.1, z.0, z.1);
        for (x2, y2, z2) in matching {
            let x_overlap = if x.0 > x2.0 { (x2.0, x.0) } else { (x.0, x2.0) };
            let y_overlap = if y.0 > y2.0 { (y2.0, y.0) } else { (y.0, y2.0) };
            let z_overlap = if z.0 > z2.0 { (z2.0, z.0) } else { (z.0, z2.0) };

            println!("    x: {}..{}, y: {}..{}, z: {}..{}", x_overlap.0, x_overlap.1, y_overlap.0, y_overlap.1, z_overlap.0, z_overlap.1)

            // let new_cubes = vec![
            //     ()  
            // ];
        }
    }

    // else {
    //     println!("\nCube: x: {}..{}, y: {}..{}, z: {}..{}", x.0, x.1, y.0, y.1, z.0, z.1);
    //     println!("Matching:");
    //     for (x, y, z) in matching {
    //         println!("  x: {}..{}, y: {}..{}, z: {}..{}", x.0, x.1, y.0, y.1, z.0, z.1);
    //     }
    // }
}

enum Dimension {
    X,
    Y,
    Z,
}

fn find_in_dimension(
    cubes: &Vec<((isize, isize), (isize, isize), (isize, isize))>,
    cube: ((isize, isize), (isize, isize), (isize, isize)),
    dimension: Dimension,
) -> Vec<((isize, isize), (isize, isize), (isize, isize))> {
    match dimension {
        Dimension::X => {
            // (x.0 < cube.0.0 && cube.0.0 < x.1) || (x.0 < cube.0.1 && cube.0.1 < x.1)

            let mut out = Vec::new();

            for old_cube in cubes {
                let (x, _, _) = old_cube;

                if (x.0 <= cube.0 .0 && cube.0 .0 <= x.1) || (x.0 <= cube.0 .1 && cube.0 .1 <= x.1)
                {
                    out.push(*old_cube);
                }
            }
            out
        }
        Dimension::Y => {
            let mut out = Vec::new();

            for old_cube in cubes {
                let (_, y, _) = old_cube;

                if (y.0 <= cube.0 .0 && cube.0 .0 <= y.1) || (y.0 <= cube.0 .1 && cube.0 .1 <= y.1)
                {
                    out.push(*old_cube);
                }
            }
            out
        }
        Dimension::Z => {
            let mut out = Vec::new();

            for old_cube in cubes {
                let (_, _, z) = old_cube;

                if (z.0 <= cube.0 .0 && cube.0 .0 <= z.1) || (z.0 <= cube.0 .1 && cube.0 .1 <= z.1)
                {
                    out.push(*old_cube);
                }
            }
            out
        }
    }
}
