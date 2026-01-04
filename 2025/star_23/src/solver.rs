use crate::parser::Input;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct TransformedShape {
    points: Vec<Point>,
    width: usize,
    height: usize,
    original_id: usize,
}

impl TransformedShape {
    fn new(grid: &[String], original_id: usize) -> Self {
        let mut points = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    points.push(Point { x: x as i32, y: y as i32 });
                }
            }
        }
        Self::normalize(points, original_id)
    }

    fn normalize(points: Vec<Point>, original_id: usize) -> Self {
        if points.is_empty() {
            return Self { points, width: 0, height: 0, original_id };
        }
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        let normalized_points: Vec<Point> = points.iter().map(|p| Point {
            x: p.x - min_x,
            y: p.y - min_y,
        }).collect();

        Self {
            points: normalized_points,
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
            original_id,
        }
    }

    fn rotate(&self) -> Self {
        // Rotate 90 degrees clockwise: (x, y) -> (-y, x)
        let points: Vec<Point> = self.points.iter().map(|p| Point {
            x: -p.y,
            y: p.x,
        }).collect();
        Self::normalize(points, self.original_id)
    }
    
    fn generate_rotations(&self) -> Vec<TransformedShape> {
        let mut rotations = Vec::new();
        let mut current = self.clone();
        
        // Try 4 rotations
        for _ in 0..4 {
            // Check if we already have this shape (to avoid duplicates for symmetric shapes)
            let mut sorted_points = current.points.clone();
            sorted_points.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
            
            let is_duplicate = rotations.iter().any(|r: &TransformedShape| {
                let mut r_points = r.points.clone();
                r_points.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
                r_points == sorted_points
            });

            if !is_duplicate {
                rotations.push(current.clone());
            }
            current = current.rotate();
        }
        rotations
    }
}

struct Solver {
    grid: Vec<bool>,
    width: usize,
    height: usize,
}

impl Solver {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![false; width * height],
            width,
            height,
        }
    }

    fn can_place(&self, shape: &TransformedShape, x: usize, y: usize) -> bool {
        if x + shape.width > self.width || y + shape.height > self.height {
            return false;
        }
        for p in &shape.points {
            let px = x + p.x as usize;
            let py = y + p.y as usize;
            if self.grid[py * self.width + px] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &TransformedShape, x: usize, y: usize, val: bool) {
        for p in &shape.points {
            let px = x + p.x as usize;
            let py = y + p.y as usize;
            self.grid[py * self.width + px] = val;
        }
    }

    fn solve_recursive(&mut self, shapes_to_place: &[Vec<TransformedShape>], last_placed_idx: Option<usize>) -> bool {
        if shapes_to_place.is_empty() {
            return true;
        }

        let current_shape_variations = &shapes_to_place[0];
        let remaining_shapes = &shapes_to_place[1..];
        
        // Symmetry breaking:
        // If the current shape is the same ID as the previous one, we can restrict positions
        // to be after the last placed position.
        let start_linear_idx = last_placed_idx.unwrap_or(0);

        // Optimization: Check if remaining area is sufficient
        let empty_cells = self.grid.iter().filter(|&&b| !b).count();
        let needed_area: usize = shapes_to_place.iter().map(|v| v[0].points.len()).sum();
        if empty_cells < needed_area {
            return false;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let linear_idx = y * self.width + x;
                if linear_idx < start_linear_idx {
                    continue;
                }

                for variant in current_shape_variations {
                    if self.can_place(variant, x, y) {
                        self.place(variant, x, y, true);
                        
                        // Check if the next shape is the same as the current one
                        let next_start_idx = if !remaining_shapes.is_empty() 
                            && remaining_shapes[0][0].original_id == current_shape_variations[0].original_id 
                        {
                            Some(linear_idx)
                        } else {
                            None
                        };

                        if self.solve_recursive(remaining_shapes, next_start_idx) {
                            return true;
                        }
                        
                        self.place(variant, x, y, false);
                    }
                }
            }
        }
        
        false
    }
}

pub fn solve(input: &Input) {
    // Pre-process shapes
    let all_shapes: Vec<Vec<TransformedShape>> = input.shapes.iter()
        .map(|s| TransformedShape::new(&s.grid, s.id).generate_rotations())
        .collect();

    let mut fitting_count = 0;
    let total_count = input.containers.len();

    for (i, container) in input.containers.iter().enumerate() {
        print!("Container {} ({}x{}): ", i, container.width, container.height);
        
        // Expand the counts into a list of shape indices to place
        let mut shapes_indices = Vec::new();
        for (shape_idx, &count) in container.shapes.iter().enumerate() {
            for _ in 0..count {
                if shape_idx < all_shapes.len() {
                    shapes_indices.push(shape_idx);
                }
            }
        }
        
        // Sort shapes by size (largest first), then by ID to group identical shapes
        shapes_indices.sort_by(|&a, &b| {
             let size_a = all_shapes[a][0].points.len();
             let size_b = all_shapes[b][0].points.len();
             size_b.cmp(&size_a).then(a.cmp(&b))
        });

        let shapes_to_place: Vec<Vec<TransformedShape>> = shapes_indices.iter()
            .map(|&idx| all_shapes[idx].clone())
            .collect();

        let mut solver = Solver::new(container.width, container.height);
        if solver.solve_recursive(&shapes_to_place, None) {
            println!("Fits!");
            fitting_count += 1;
        } else {
            println!("Does not fit.");
        }
    }

    println!("\nSummary: {}/{} containers fit.", fitting_count, total_count);
}
