use std::collections::HashMap;

use data::data::INPUT;

mod data;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Segment {
    fn new(start: Point, end: Point) -> Segment {
        Segment { start, end }
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

#[derive(Debug)]
enum Direction {
    Positive,
    Negative
}

#[derive(Debug)]
struct SegmentIter<'a> {
    segment: &'a Segment,
    x_direction: Direction,
    y_direction: Direction,
    current: Option<Point>,
}

impl<'a> SegmentIter<'a> {
    fn new(segment: &Segment) -> SegmentIter {
        SegmentIter {
            segment,
            x_direction: {
                if segment.start.x < segment.end.x {
                    Direction::Positive
                } else {
                    Direction::Negative
                }
            },
            y_direction: {
                if segment.start.y < segment.end.y {
                    Direction::Positive
                } else {
                    Direction::Negative
                }
            },
            current: None,
        }
    }
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                self.current = Some(self.segment.start);
                self.current
            }
            Some(point) => {
                if point == self.segment.end {
                    None
                } else if self.segment.start.x == self.segment.end.x {
                    let next_point = Point {
                        x: point.x,
                        y: {
                            match self.y_direction {
                                Direction::Positive => point.y + 1,
                                Direction::Negative => point.y - 1,
                            }
                        },
                    };
                    self.current = Some(next_point);
                    self.current
                } else if self.segment.start.y == self.segment.end.y {
                    let next_point = Point {
                        x: {
                            match self.x_direction {
                                Direction::Positive => point.x + 1,
                                Direction::Negative => point.x - 1,
                            }
                        },
                        y: point.y,
                    };
                    self.current = Some(next_point);
                    self.current
                } else {
                    let next_point = Point {
                        x: {
                            match self.x_direction {
                                Direction::Positive => point.x + 1,
                                Direction::Negative => point.x - 1,
                            }
                        },
                        y: {
                            match self.y_direction {
                                Direction::Positive => point.y + 1,
                                Direction::Negative => point.y - 1,
                            }
                        },
                    };
                    self.current = Some(next_point);
                    self.current
                }
            }
        }
    }
}

fn main() {
    let input = INPUT.trim().split('\n').into_iter();

    let segments: Vec<Segment> = input.map(parse_segment).collect();

    let mut vents = HashMap::new();

    segments
        .iter()
        //.filter(|&segment| segment.is_straight())
        .for_each(|segment| {
            SegmentIter::new(segment).for_each(|point| {
                let count = vents.entry(point).or_insert(0);
                *count += 1;
            });
        });
    
    let result = vents.into_values()
    .filter(|value| *value > 1)
    .count();

    println!("2 lines overlaps on {} points", result);
}

fn parse_segment(segment: &str) -> Segment {
    let mut parts = segment.split("->").into_iter();
    let start = parse_point(parts.next().unwrap());
    let end = parse_point(parts.next().unwrap());
    Segment {
        start,
        end,
    }
}

fn parse_point(point: &str) -> Point {
    let mut parts = point.trim().split(',').into_iter();

    let x: i32 = parts.next().unwrap().parse().unwrap();
    let y: i32 = parts.next().unwrap().parse().unwrap();

    Point { x, y }
}
