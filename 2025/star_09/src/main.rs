mod parser;

use parser::parse_input;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    let parsed = parse_input(&input);

    println!("Fresh ingredients: {:?}", parsed.fresh_ingredients);
    println!("Available ingredients: {:?}", parsed.available_ingredients);

    let count = parsed
        .available_ingredients
        .iter()
        .filter(|&ingredient| {
            parsed
                .fresh_ingredients
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count();

    println!("Available ingredients in fresh ranges: {}", count);
    
}
