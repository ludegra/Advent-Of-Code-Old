use std::collections::HashMap;

use crate::structs::Cave;

pub fn part1(caves: &HashMap<String, Cave>) {
    let paths = check_path(caves, "start", Vec::new()).unwrap();
    println!("Part 1: {}", paths.len());
}

fn check_path(
    caves: &HashMap<String, Cave>,
    cave_name: &str,
    mut visited_small_caves: Vec<String>,
) -> Option<Vec<Vec<String>>> {
    if cave_name == "end" {
        return None;
    }

    let cave = caves.get(cave_name).unwrap();

    if cave_name.to_lowercase() == cave_name {
        visited_small_caves.push(cave_name.to_string());
    }

    let mut paths = Vec::new();

    for connection in cave.get_connections() {
        if !visited_small_caves.contains(connection) {
            let sub_paths = check_path(caves, connection, visited_small_caves.clone());
            
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
