use colored::Colorize;

use crate::structs::Octopus;

pub fn part2(input: &Vec<Vec<u32>>) {
    let mut octogrid = Vec::with_capacity(input.len());

    for y in 0..input.len() {
        let mut octorow = Vec::new();
        for x in 0..input[0].len() {
            let octopus = Octopus::new(x, y, &mut octogrid, input[y][x]);
            octorow.push(octopus);
        }
        octogrid.push(octorow);
    }

    let mut flash_counter = 0;

    // octogrid.print();
    for i in 1.. {
        for octorow in &mut octogrid {
            for octopus in octorow {
                octopus.increase_energy()
            }
        }
        let mut number_flashed = 0;

        let mut has_flashed = true;
        while has_flashed {
            has_flashed = false;

            for octorow in &mut octogrid {
                for octopus in octorow {
                    if let Some(()) = octopus.flash() {
                        has_flashed = true;
                        number_flashed += 1;
                    }
                }
            }
        }

        if number_flashed == octogrid.len() * octogrid[0].len() {
            println!("Part 2: {}", i);
            return ()
        }

        for octorow in &mut octogrid {
            for octopus in octorow {
                octopus.finish_round(&mut flash_counter);
            }
        }

        // octogrid.print();
    }
}

trait Print {
    fn print(&self);
}
impl Print for Vec<Vec<Octopus<'_>>> {
    fn print(&self) {
        for octorow in self {
            for octopus in octorow {
                if octopus.energy == 0 {
                    print!("{}", octopus.energy.to_string().yellow())
                } else {
                    print!("{}", octopus.energy);
                }
            }
            println!("");
        }
        println!("");
    }
}