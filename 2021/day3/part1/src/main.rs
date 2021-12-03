use std::collections::{BTreeMap, HashMap};
use std::fs::{File, read_to_string};
use std::ptr::hash;

fn main() -> anyhow::Result<()> {
    let mut hashmap: BTreeMap<usize, i64> = BTreeMap::new();
    read_to_string("data/input.txt")?
        .lines()
        .for_each(|line| {
            line.chars().enumerate().for_each(|(index, char)| {
                *hashmap.entry(index).or_insert(0) += if char == '1' { 1 } else { -1 };
            });
        });
    dbg!(&hashmap);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    hashmap.iter().for_each(|(index, &count)| {
        gamma.push(if count > 0 { '1' } else { '0' });
        epsilon.push(if count < 0 { '1' } else { '0' })
    });
    let gamma_int = isize::from_str_radix(gamma.as_ref(), 2).unwrap();
    let epsilon_int = isize::from_str_radix(epsilon.as_ref(), 2).unwrap();
    dbg!(gamma_int, epsilon_int, gamma_int * epsilon_int);
    Ok(())
}
