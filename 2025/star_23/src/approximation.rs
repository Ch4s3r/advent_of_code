use crate::parser;

pub fn solve(parsed: &parser::Input) {
    let (yes, no) = parsed.containers.iter().fold((0, 0), |(yes, no), container| {
        let grid_size = container.width * container.height;
        let cells_needed: usize = container.shapes.iter().map(|&count| count * 9).sum();
        
        if cells_needed <= grid_size {
            (yes + 1, no)
        } else {
            (yes, no + 1)
        }
    });
    
    println!("Summary: {}/{} containers fit.", yes, yes + no);
}
