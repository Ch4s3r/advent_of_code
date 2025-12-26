mod parser;

use parser::{parse_points, Point};
use std::fs;
use indicatif::ProgressBar;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (_, points) = parse_points(&input).unwrap();

    let answer = find_largest_rectangle(&points);
    println!("Part 2: {}", answer);
}

fn find_largest_rectangle(points: &[Point]) -> u64 {
    let mut max_area = 0;
    
    // Pre-compute red set once
    let red_set: HashSet<(u32, u32)> = 
        points.iter().map(|p| (p.x, p.y)).collect();
    
    let total_pairs = (points.len() * (points.len() - 1) / 2) as u64;
    let pb = ProgressBar::new(total_pairs);

    // Try all pairs of red tiles as opposite corners
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pb.inc(1);
            
            let p1 = &points[i];
            let p2 = &points[j];

            // Check if this pair forms a valid rectangle
            // Red tiles must be in opposite corners
            // The rectangle is axis-aligned
            let x1 = p1.x.min(p2.x);
            let x2 = p1.x.max(p2.x);
            let y1 = p1.y.min(p2.y);
            let y2 = p1.y.max(p2.y);

            // Check if this rectangle is valid and has red corners
            if has_red_corners(&red_set, x1, y1, x2, y2) && 
               is_rectangle_inside_polygon(x1, y1, x2, y2, points) {
                let area = ((x2 - x1 + 1) as u64) * ((y2 - y1 + 1) as u64);
                max_area = max_area.max(area);
            }
        }
    }
    
    pb.finish_with_message("Done!");

    max_area
}


fn has_red_corners(red_set: &HashSet<(u32, u32)>, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    // Check all four corners - must have at least two red tiles in opposite corners
    let corners = vec![
        (x1, y1),
        (x1, y2),
        (x2, y1),
        (x2, y2),
    ];

    let red_corner_count = corners.iter().filter(|c| red_set.contains(c)).count();
    red_corner_count >= 2
}

fn is_rectangle_inside_polygon(x1: u32, y1: u32, x2: u32, y2: u32, polygon: &[Point]) -> bool {
    // Check all four corners - must all be inside or on polygon
    let corners = vec![(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    
    for &(x, y) in &corners {
        if !point_in_or_on_polygon(x, y, polygon) {
            return false;
        }
    }
    
    // Check edges at regular intervals for large rectangles
    let step = if (x2 - x1).max(y2 - y1) > 100 { 10 } else { 1 };
    
    // Check top and bottom edges
    for x in (x1..=x2).step_by(step as usize) {
        if !point_in_or_on_polygon(x, y1, polygon) || !point_in_or_on_polygon(x, y2, polygon) {
            return false;
        }
    }
    
    // Check left and right edges
    for y in (y1..=y2).step_by(step as usize) {
        if !point_in_or_on_polygon(x1, y, polygon) || !point_in_or_on_polygon(x2, y, polygon) {
            return false;
        }
    }
    
    // For smaller rectangles, also check interior
    if (x2 - x1) <= 20 && (y2 - y1) <= 20 {
        for x in x1..=x2 {
            for y in y1..=y2 {
                if !point_in_or_on_polygon(x, y, polygon) {
                    return false;
                }
            }
        }
    }
    
    true
}

fn point_in_or_on_polygon(x: u32, y: u32, polygon: &[Point]) -> bool {
    // First check if point is on the boundary
    for i in 0..polygon.len() {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % polygon.len()];
        
        // Check if point is on the line segment from p1 to p2
        if is_point_on_segment(x, y, p1, p2) {
            return true;
        }
    }
    
    // If not on boundary, check if inside using ray casting
    point_inside_polygon(x, y, polygon)
}

fn is_point_on_segment(x: u32, y: u32, p1: &Point, p2: &Point) -> bool {
    // Check if point (x,y) is on the line segment from p1 to p2
    let x_min = p1.x.min(p2.x);
    let x_max = p1.x.max(p2.x);
    let y_min = p1.y.min(p2.y);
    let y_max = p1.y.max(p2.y);
    
    // Point must be within bounding box of segment
    if x < x_min || x > x_max || y < y_min || y > y_max {
        return false;
    }
    
    // Check if point is on horizontal or vertical segment
    (p1.x == p2.x && x == p1.x) || (p1.y == p2.y && y == p1.y)
}

fn point_inside_polygon(x: u32, y: u32, polygon: &[Point]) -> bool {
    // Ray casting algorithm: count intersections with polygon edges
    let mut crossings = 0;
    
    for i in 0..polygon.len() {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % polygon.len()];
        
        // Cast a ray to the right (positive x direction) from the point
        // Count how many edges it crosses
        
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;
        
        // Check if the edge crosses the horizontal ray
        if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
            // Calculate x-coordinate of intersection
            let x_intersect = x1 as f64 + (y as f64 - y1 as f64) * (x2 as f64 - x1 as f64) / (y2 as f64 - y1 as f64);
            
            if (x as f64) < x_intersect {
                crossings += 1;
            }
        }
    }
    
    // Odd number of crossings means point is inside
    crossings % 2 == 1
}