use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use rayon::iter::IndexedParallelIterator;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n\n").collect();
    let parser_rules = parser!(
        lines(u64 "|" u64)
    );
    let parser_ordering = parser!(lines(repeat_sep(u64, ",")));
    let input_rules = parser_rules.parse(split[0]).unwrap();
    let input_ordering = parser_ordering.parse(split[1]).unwrap();

    let incorrect_orderings: Vec<Vec<_>> = input_ordering
        .into_iter()
        .enumerate()
        .filter_map(|(odering_index, ordering)| {
            let ordering_correct = ordering.iter().enumerate().all(|(index, page)| {
                let not_before_current = input_rules
                    .iter()
                    .filter_map(|(key, value)| if key == page { Some(value) } else { None })
                    .collect::<HashSet<_>>();
                let pages_before: HashSet<_> = HashSet::from_iter(&ordering[0..index]);
                let correct = pages_before
                    .intersection(&not_before_current)
                    .collect::<HashSet<_>>()
                    .is_empty();
                correct
            });
            if ordering_correct {
                None
            } else {
                Some(ordering)
            }
        })
        .collect_vec();
    dbg!(&incorrect_orderings.len());

    let sorted_pages: Vec<_> = incorrect_orderings
        .iter()
        .map(|ordering| {
            let mut ordering = ordering.clone();
            let mut new_ordering = ordering.clone();
            loop {
                ordering = new_ordering.clone();
                let mut all_pages_correct = true;
                for (index, page) in ordering.iter().enumerate() {
                    let not_before_current = input_rules
                        .iter()
                        .filter_map(|(key, value)| if key == page { Some(value) } else { None })
                        .collect::<HashSet<_>>();
                    let pages_before: HashSet<_> = HashSet::from_iter(&ordering[0..index]);
                    let intersection = pages_before
                        .intersection(&not_before_current)
                        .collect::<Vec<_>>();
                    let correct = intersection.is_empty();
                    if !correct {
                        let item_to_remove: u64 = ***intersection.first().unwrap();
                        dbg!("before", &ordering, &item_to_remove, &page);
                        let ordering_index_of_item_to_remove =
                            ordering.iter().position(|x| x == &item_to_remove).unwrap();
                        let item_to_add_at_the_end =
                            new_ordering.remove(ordering_index_of_item_to_remove);
                        new_ordering.insert(index, item_to_add_at_the_end);
                        dbg!("after reordering", &ordering);
                        all_pages_correct = false;
                    }
                }
                if all_pages_correct {
                    dbg!(&ordering);
                    println!("new ordering correct {:?}", new_ordering,);
                    let median = new_ordering.len() / 2;
                    let median_element = new_ordering[median];
                    return median_element;
                }
            }
        })
        .collect();
    let score: u64 = sorted_pages.iter().sum();
    dbg!(score);
}
// 75|47
// 29|13
