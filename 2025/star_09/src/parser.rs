#[derive(Debug, Clone)]
pub struct RecipeData {
    pub fresh_ingredients: Vec<std::ops::RangeInclusive<u64>>,
    pub available_ingredients: Vec<u64>,
}

fn parse_range(range_str: &str) -> Option<std::ops::RangeInclusive<u64>> {
    if let Some((start, end)) = range_str.split_once('-') {
        if let (Ok(s), Ok(e)) = (start.parse::<u64>(), end.parse::<u64>()) {
            return Some(s..=e);
        }
    }
    None
}

pub fn parse_input(input: &str) -> RecipeData {
    let input = input.trim_end();
    
    // Split by blank line
    let (fresh_part, available_part) = input.split_once("\n\n")
        .expect("Input must contain a blank line separator");
    
    // Parse fresh ingredients into ranges
    let fresh_ingredients: Vec<std::ops::RangeInclusive<u64>> = fresh_part
        .lines()
        .filter_map(parse_range)
        .collect();
    
    // Parse available ingredients (numbers)
    let available_ingredients: Vec<u64> = available_part
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    RecipeData {
        fresh_ingredients,
        available_ingredients,
    }
}
