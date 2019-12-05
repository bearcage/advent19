use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Overlap {
    Point(i64, i64),
    Range(LineSegment),
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct LineSegment {
    pub start: (i64, i64),
    pub end: (i64, i64),
}
impl LineSegment {
    fn coaxial_overlap(&self, other: &LineSegment) -> Option<Overlap> {
        let horizontal = self.start.1 == self.end.1;
        debug_assert_eq!(horizontal, other.start.1 == other.end.1);

        let (start_1, end_1, start_2, end_2);
        if horizontal {
            if self.start.1 != other.start.1 {
                return None;
            }
            start_1 = self.start.0;
            end_1 = self.end.0;
            start_2 = other.start.0;
            end_2 = other.end.0;
        } else {
            if self.start.0 != other.start.0 {
                return None;
            }
            start_1 = self.start.1;
            end_1 = self.end.1;
            start_2 = other.start.1;
            end_2 = other.end.1;
        }

        let mut points = vec![
            (0, 0, start_1),
            (0, 1, end_1),
            (1, 0, start_2),
            (1, 1, end_2),
        ];
        points.sort_by_key(|p| p.2);
        if points[0].0 == points[1].0 {
            return None;
        }

        let overlap_start = points[1].2;
        let overlap_end = points[2].2;
        let segment = if horizontal {
            LineSegment {
                start: (overlap_start, self.start.1),
                end: (overlap_end, self.end.1),
            }
        } else {
            LineSegment {
                start: (self.start.0, overlap_start),
                end: (self.end.0, overlap_end),
            }
        };

        Some(Overlap::Range(segment))
    }
    fn perpendicular_overlap(&self, other: &LineSegment) -> Option<Overlap> {
        let (fixed_x, min_x, max_x);
        let (fixed_y, min_y, max_y);

        let self_horizontal = self.start.1 == self.end.1;
        if self_horizontal {
            debug_assert!(other.start.0 == other.end.0);
            fixed_x = other.start.0;
            min_x = min(self.start.0, self.end.0);
            max_x = max(self.start.0, self.end.0);

            fixed_y = self.start.1;
            min_y = min(other.start.1, other.end.1);
            max_y = max(other.start.1, other.end.1);
        } else {
            debug_assert!(other.start.1 == other.end.1);
            fixed_x = self.start.0;
            min_x = min(other.start.0, other.end.0);
            max_x = max(other.start.0, other.end.0);

            fixed_y = other.start.1;
            min_y = min(self.start.1, self.end.1);
            max_y = max(self.start.1, self.end.1);
        }

        let in_x_range = min_x <= fixed_x && fixed_x <= max_x;
        if !in_x_range {
            return None;
        }
        let in_y_range = min_y <= fixed_y && fixed_y <= max_y;
        if !in_y_range {
            return None;
        }
        let intersection = Overlap::Point(fixed_x, fixed_y);
        Some(intersection)
    }
    pub fn overlap(&self, other: &LineSegment) -> Option<Overlap> {
        let self_horizontal = self.start.1 == self.end.1;
        let other_horizontal = other.start.1 == other.end.1;

        let coaxial = self_horizontal == other_horizontal;

        if coaxial {
            self.coaxial_overlap(other)
        } else {
            self.perpendicular_overlap(other)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct WireVector {
    pub direction: Direction,
    pub magnitude: i64,
}
pub trait VectorSequence {
    fn to_line_segments(&self) -> Vec<LineSegment>;
}
impl<'a> VectorSequence for &'a [WireVector] {
    fn to_line_segments(&self) -> Vec<LineSegment> {
        let init = (0, 0, Vec::new());
        let (_, _, segments) =
            self.iter().fold(init, |(x1, y1, mut acc), elem| {
                let WireVector {
                    direction,
                    magnitude,
                } = elem;
                let (x2, y2) = match direction {
                    Direction::Up => (x1, y1 + magnitude),
                    Direction::Down => (x1, y1 - magnitude),
                    Direction::Right => (x1 + magnitude, y1),
                    Direction::Left => (x1 - magnitude, y1),
                };

                acc.push(LineSegment {
                    start: (x1, y1),
                    end: (x2, y2),
                });

                (x2, y2, acc)
            });
        segments
    }
}
