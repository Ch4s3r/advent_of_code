use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const MANDATORY_ATTRIBUTES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
lazy_static! {
    static ref HGT_REGEX: Regex = Regex::new(r"(\w+)(in|cm)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\w{9}$").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let start = Instant::now();

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
    // dbg!(passport_strings.clone());

    let mut valid_count = 0;
    for passport in passport_strings.iter() {
        let passport = passport.split_ascii_whitespace().collect::<Vec<&str>>();
        let passport_tuples = passport
            .iter()
            .map(|attribute| {
                let split_attribute = attribute.split(":").collect::<Vec<&str>>();
                (split_attribute[0], split_attribute[1])
            })
            .collect::<HashMap<&str, &str>>();

        let mut valid_passport_attributes = 0;
        for attribute in MANDATORY_ATTRIBUTES.iter() {
            if !passport_tuples.contains_key(attribute) {
                // dbg!(attribute);
                break;
            }
            let attribute_value = passport_tuples[attribute];
            // dbg!(attribute, attribute_value);

            match *attribute {
                "byr" => {
                    let year = attribute_value.parse::<i32>().unwrap_or(0);
                    if (1920..=2020).contains(&year) {
                        valid_passport_attributes += 1
                    }
                }
                "iyr" => {
                    let year = attribute_value.parse::<i32>().unwrap_or(0);
                    if (2010..=2020).contains(&year) {
                        valid_passport_attributes += 1
                    }
                }
                "eyr" => {
                    let year = attribute_value.parse::<i32>().unwrap_or(0);
                    if (2020..=2030).contains(&year) {
                        valid_passport_attributes += 1
                    }
                }
                "hgt" => {
                    let capture = match HGT_REGEX.captures(attribute_value) {
                        Some(x) => x,
                        _ => break,
                    };
                    let unit = capture[2].to_string();
                    let number = capture[1].parse::<i32>().unwrap();
                    if unit == "cm" {
                        if (150..=193).contains(&number) {
                            valid_passport_attributes += 1
                        }
                    } else if unit == "in" {
                        if (59..=76).contains(&number) {
                            valid_passport_attributes += 1
                        }
                    }
                }
                "hcl" => {
                    if HCL_REGEX.is_match(attribute_value) {
                        valid_passport_attributes += 1
                    }
                }
                "ecl" => {
                    if ECL_REGEX.is_match(attribute_value) {
                        valid_passport_attributes += 1
                    }
                }
                "pid" => {
                    if PID_REGEX.is_match(attribute_value) {
                        valid_passport_attributes += 1
                    }
                }
                _ => {}
            };
        }
        if valid_passport_attributes == 7 {
            valid_count += 1;
        }
    }

    dbg!(start.elapsed());
    dbg!(valid_count);
    Ok(())
}
