use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MANDATORY_ATTRIBUTES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut concat: Vec<String> = Vec::new();
    let mut passport_strings: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            passport_strings.push(concat.join(" "));
            concat.clear();
        } else {
            concat.push(line);
        }
    }
    passport_strings.push(concat.join(" "));
    dbg!(passport_strings.clone());

    let mut valid_count = 0;
    for passport in passport_strings.iter() {
        let passport = passport.split_ascii_whitespace().collect::<Vec<&str>>();
        let passport_tuples = passport
            .iter()
            .map(|attribute| {
                let split_attribute = attribute.split(":").collect::<Vec<&str>>();
                (split_attribute[0].clone(), split_attribute[1].clone())
            })
            .collect::<HashMap<&str, &str>>();
        let mut valid_passport = true;
        for attribute in MANDATORY_ATTRIBUTES.iter() {
            if !passport_tuples.contains_key(attribute) {
                valid_passport = false
            }
        }
        if valid_passport {
            valid_count += 1;
        }
    }

    dbg!(valid_count);
    Ok(())
}
