use std::collections::HashMap;

pub fn part2(input: &Vec<(bool, (isize, isize), (isize, isize), (isize, isize))>) {
    // let map = HashMap::new();

    // let mut intertwined = false;
}

enum Overlapping {
    Left,
    Right,
    Enclosed,
    Encloses,
}

fn split_cube(
    cube: ((isize, isize), (isize, isize), (isize, isize)),
    intertwined: ((isize, isize), (isize, isize), (isize, isize)),
) -> Option<Vec<((isize, isize), (isize, isize), (isize, isize))>> {
    // Splitting into more easily worked-with variables
    let ((x_start_cube, x_end_cube), (y_start_cube, y_end_cube), (z_start_cube, z_end_cube)) = cube;
    let (
        (x_start_intersecting, x_end_intersecting),
        (y_start_intersecting, y_end_intersecting),
        (z_start_intersecting, z_end_intersecting),
    ) = intertwined;

    // Checkning relative positions
    let x_overlap = match type_of_overlap(
        x_start_cube,
        x_end_cube,
        x_start_intersecting,
        x_end_intersecting,
    ) {
        Some(overlap) => overlap,
        None => return None,
    };
    let y_overlap = match type_of_overlap(
        y_start_cube,
        y_end_cube,
        y_start_intersecting,
        y_end_intersecting,
    ) {
        Some(overlap) => overlap,
        None => return None,
    };
    let z_overlap = match type_of_overlap(
        z_start_cube,
        z_end_cube,
        z_start_intersecting,
        z_end_intersecting,
    ) {
        Some(overlap) => overlap,
        None => return None,
    };

    match x_overlap {
        Overlapping::Left => {}
        Overlapping::Right => {}
        Overlapping::Enclosed => {}
        Overlapping::Encloses => match y_overlap {
            Overlapping::Left => {}
            Overlapping::Right => {}
            Overlapping::Enclosed => match z_overlap {
                Overlapping::Left => {}
                Overlapping::Right => {}
                Overlapping::Enclosed => {
                    return Some(vec![
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_end_cube),
                            (z_start_cube, z_start_intersecting - 1),
                        ),
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_end_cube),
                            (z_end_intersecting + 1, z_end_cube),
                        ),
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_start_intersecting - 1),
                            (z_start_intersecting, z_end_intersecting),
                        ),
                        (
                            (x_start_cube, x_end_cube),
                            (y_end_intersecting + 1, y_end_cube),
                            (z_start_intersecting, z_end_intersecting),
                        ),
                    ])
                }
                Overlapping::Encloses => {
                    return Some(vec![
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_start_intersecting - 1),
                            (z_start_cube, z_end_cube),
                        ),
                        (
                            (x_start_cube, x_end_cube),
                            (y_end_intersecting + 1, y_end_cube),
                            (z_start_cube, z_end_cube),
                        ),
                    ])
                }
            },
            Overlapping::Encloses => match z_overlap {
                Overlapping::Left => {
                    return Some(vec![(
                        (x_start_cube, x_end_cube),
                        (y_start_cube, y_end_cube),
                        (z_end_intersecting + 1, z_end_cube),
                    )])
                }
                Overlapping::Right => {
                    return Some(vec![(
                        (x_start_cube, x_end_cube),
                        (y_start_cube, y_end_cube),
                        (z_start_cube, z_start_intersecting - 1),
                    )])
                }
                Overlapping::Enclosed => {
                    return Some(vec![
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_end_cube),
                            (z_start_cube, z_start_intersecting - 1),
                        ),
                        (
                            (x_start_cube, x_end_cube),
                            (y_start_cube, y_end_cube),
                            (z_end_intersecting + 1, z_end_cube),
                        ),
                    ])
                }
                Overlapping::Encloses => return Some(Vec::new()),
            },
        },
    }

    None
}

fn type_of_overlap(
    start_cube: isize,
    end_cube: isize,
    start_intersecting: isize,
    end_intersecting: isize,
) -> Option<Overlapping> {
    let overlap: Overlapping;

    // IIIIII
    //    XXXXXXX
    if start_intersecting < start_cube
        && end_intersecting > start_cube
        && end_intersecting < end_cube
    {
        overlap = Overlapping::Left;
    }
    //   IIIIIII
    // XXXXX
    else if start_cube < start_intersecting
        && end_cube > start_intersecting
        && end_cube < end_intersecting
    {
        overlap = Overlapping::Right;
    }
    //  IIIIII
    // XXXXXXXXXX
    else if start_cube < start_intersecting && end_cube > end_intersecting {
        overlap = Overlapping::Enclosed;
    }
    // IIIIIIIIIIIIIII
    //   XXXXXXXX
    else if start_cube > start_intersecting && end_cube < end_intersecting {
        overlap = Overlapping::Enclosed;
    } else {
        return None;
    }
    Some(overlap)
}
