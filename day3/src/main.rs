pub mod input;
pub mod parser;
pub mod types;

use types::{Overlap, VectorSequence};

pub fn origin_manhattan(overlap: Overlap) -> Option<usize> {
    match overlap {
        Overlap::Point(0, 0) => None,
        Overlap::Point(x, y) => Some((x.abs() + y.abs()) as usize),
        Overlap::Range(segment) => {
            let x = std::cmp::min(segment.start.0.abs(), segment.end.0.abs());
            let y = std::cmp::min(segment.start.1.abs(), segment.end.1.abs());
            if x == 0 && y == 0 {
                return None;
            }
            Some((x + y) as usize)
        },
    }
}

fn main() {
    let wire_specs: Vec<_> =
        input::INPUT.lines().filter(|l| !l.is_empty()).collect();
    debug_assert_eq!(wire_specs.len(), 2);

    // Parse both wires
    let wire1 = wire_specs[0];
    let (rest, vec1) =
        parser::parse_vectors(wire1).expect("first wire parse failed!");
    debug_assert!(rest.is_empty());
    let wire2 = wire_specs[1];
    let (rest, vec2) =
        parser::parse_vectors(wire2).expect("second wire parse failed!");
    debug_assert!(rest.is_empty());

    // Extract line segments from both wires
    let seg1 = vec1.as_slice().to_line_segments();
    let seg2 = vec2.as_slice().to_line_segments();

    // Naive O(n^2) intersection testing routine
    let mut intersections = Vec::new();
    for s1 in seg1.iter() {
        for s2 in seg2.iter() {
            match s1.overlap(&s2) {
                Some(point) => intersections.push(point),
                None => {},
            }
        }
    }

    let mut shortest_circuit = std::usize::MAX;
    for intersection in intersections {
        match intersection {
            Overlap::Point(x, y) => {
                let d1 = vec1
                    .as_slice()
                    .parametric_distance_to(x, y)
                    .expect("invalid intersection (missing on wire 1)");
                let d2 = vec2
                    .as_slice()
                    .parametric_distance_to(x, y)
                    .expect("invalid intersection (missing on wire 2)");
                shortest_circuit = std::cmp::min(shortest_circuit, d1 + d2);
            },
            _ => { /* noop for now */ },
        }
    }

    println!("shortest circuit: {}", shortest_circuit);
}
