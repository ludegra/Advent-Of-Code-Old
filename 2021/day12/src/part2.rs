use std::collections::HashMap;

use crate::structs::Cave;

pub fn part2(caves: &HashMap<String, Cave>) {
    let paths = check_path(caves, "start", Vec::new(), false).unwrap();
    println!("Part 2: {}", paths.len());
}

fn check_path(
    caves: &HashMap<String, Cave>,
    cave_name: &str,
    mut visited_small_caves: Vec<String>,
    mut small_cave_visited_twice: bool
) -> Option<Vec<Vec<String>>> {
    if cave_name == "end" {
        return None;
    }

    let cave = caves.get(cave_name).unwrap();

    if cave_name.to_lowercase() == cave_name {
        if visited_small_caves.contains(&cave_name.to_string()) && cave_name != "start" {
            small_cave_visited_twice = true;
        }

        visited_small_caves.push(cave_name.to_string());
    }

    let mut paths = Vec::new();

    for connection in cave.get_connections() {
        let small_cave_visited = visited_small_caves.contains(connection);

        if (!small_cave_visited || !small_cave_visited_twice) && connection != "start" {

            let sub_paths = check_path(caves, connection, visited_small_caves.clone(), small_cave_visited_twice);
            
            match sub_paths {
                Some(mut sub_paths) => {
                    let mut path = vec![vec![cave_name.to_string()]; sub_paths.len()];
                    for i in 0..sub_paths.len() {
                        path[i].append(&mut sub_paths[i])
                    }
                    paths.append(&mut path);
                }
                None => {
                    paths.append(&mut vec![vec![cave_name.to_string(), "end".to_string()]])
                }
            }
        }
    }

    Some(paths)
}
