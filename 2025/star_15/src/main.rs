mod parser;

use itertools::Itertools;
use parser::parse_points;
use std::{collections::HashMap, fs};

use crate::parser::Point3D;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let junctions = parse_points(&input).unwrap().1;
    let mut distances = junctions
        .iter()
        .combinations(2)
        .map(|pair| {
            let dist = euclidean_distance(pair[0], pair[1]);
            ((pair[0].clone(), pair[1].clone()), dist)
        })
        .collect::<Vec<_>>();
    // dbg!(distances.len());
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let junctions_to_connect = distances.iter().collect_vec();

    let mut circuits: HashMap<Point3D, u32> = HashMap::new();
    let mut circuit_count = 0;
    for ((junction1, junction2), _) in junctions_to_connect.iter().take(1000) {
        let junction1_value = circuits.get(junction1).cloned();
        let junction2_value = circuits.get(junction2).cloned();
        if junction1_value.is_some() && junction1_value == junction2_value {
            // dbg!("Already connected junctions: {:?} and {:?}", junction1, junction2);
            continue;
        } else if let Some(junction1_value) = junction1_value
            && junction2_value.is_none()
        {
            circuits.insert(junction2.clone(), junction1_value);
        } else if let Some(junction2_value) = junction2_value
            && junction1_value.is_none()
        {
            circuits.insert(junction1.clone(), junction2_value);
        } else if junction1_value.is_none() && junction2_value.is_none() {
            circuit_count += 1;
            circuits.insert(junction1.clone(), circuit_count);
            circuits.insert(junction2.clone(), circuit_count);
        } else if let Some(junction1_value) = junction1_value
            && let Some(junction2_value) = junction2_value
            && junction1_value != junction2_value
        {
            for (_, value) in circuits.iter_mut() {
                if *value == junction2_value {
                    *value = junction1_value;
                }
                // dbg!("Merging circuits at point: {:?} from {} to {}", point, junction2_value, junction1_value);
            }
        }
    }
    let grouped = circuits.values().fold(HashMap::new(), |mut acc, &value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    });
    let mut sorted: Vec<_> = grouped.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    dbg!(&sorted);
    let result = sorted
        .iter()
        .take(3)
        .map(|&(_, count)| count)
        .product::<u128>();
    println!("{}", result);
}

fn euclidean_distance(point1: &Point3D, point2: &Point3D) -> f64 {
    ((point1.x - point2.x).pow(2) + (point1.y - point2.y).pow(2) + (point1.z - point2.z).pow(2))
        .isqrt() as f64
}
