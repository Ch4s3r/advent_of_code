mod parser;

use parser::parse_input;
use range_collections::{RangeSet2, range_set::RangeSetRange};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let parsed = parse_input(&input);

    let mut all_ingredients: RangeSet2<u64> = RangeSet2::from(0..0);

    for range in &parsed.fresh_ingredients {
        let temp_range: RangeSet2<u64> = RangeSet2::from(*range.start()..*range.end() + 1);
        all_ingredients.union_with(&temp_range);
    }

    let total = all_ingredients
        .iter()
        .map(|range_set_range| match range_set_range {
            RangeSetRange::Range(range) => *range.end - *range.start,
            _ => 0,
        })
        .sum::<u64>();
    dbg!(&total);
}
