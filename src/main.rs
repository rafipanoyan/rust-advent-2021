use std::{collections::HashSet, fmt::Display, str::Chars, sync::Arc};

use data::data::{INPUT, INPUT_DEBUG};

mod data;

type Signal = char;

#[derive(Debug)]
struct Entry(Vec<Vec<Signal>>, Vec<Vec<Signal>>);

impl Entry {}

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
enum Position {
    Top,
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomRight,
    Bottom,
    Unkown,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
struct Segment {
    position: Position,
    value: char,
}

impl Segment {
    fn from_ref(segments: &Vec<Segment>, value: &char) -> Segment {
        segments.iter().find(|s| s.value == *value).unwrap().clone()
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}\n", self.position, self.value)
    }
}

impl Eq for Segment {}

#[derive(Debug, Clone, PartialEq)]
struct Digit(HashSet<Segment>);

impl Digit {
    fn from_char() -> DigitBuilderFromChar {
        DigitBuilderFromChar {
            segments: Vec::new(),
        }
    }

    fn from_segment() -> DigitBuilderFromSegment {
        DigitBuilderFromSegment {
            segments: Vec::new(),
        }
    }

    fn has_segment(&self, segment: &Segment) -> bool {
        !self
            .0
            .iter()
            .filter(|&seg| seg.value == segment.value)
            .map(clone_segment)
            .collect::<Vec<Segment>>()
            .is_empty()
    }

    fn find_pos(&self, value: char) -> Position {
        self.0
            .iter()
            .filter(|&seg| seg.value == value)
            .map(clone_segment)
            .collect::<Vec<Segment>>()
            .first()
            .unwrap()
            .position
            .clone()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut word = String::new();
        self.0.iter().for_each(|s| word.push(s.value));
        write!(f, "{}", word)
    }
}

trait DigitBuilder<T> {
    fn add_all(&mut self, segments: &Vec<T>) -> &dyn DigitBuilder<T>;
    fn build(&self) -> Digit;
}

struct DigitBuilderFromChar {
    segments: Vec<char>,
}

impl DigitBuilder<char> for DigitBuilderFromChar {
    fn add_all(&mut self, segments: &Vec<char>) -> &dyn DigitBuilder<char> {
        self.segments = segments.clone();
        self
    }

    fn build(&self) -> Digit {
        let mut hash_set = HashSet::new();
        for segment in &self.segments {
            hash_set.insert(Segment {
                position: Position::Unkown,
                value: segment.clone(),
            });
        }
        Digit(hash_set)
    }
}

struct DigitBuilderFromSegment {
    segments: Vec<Segment>,
}

impl DigitBuilder<Segment> for DigitBuilderFromSegment {
    fn add_all(&mut self, segments: &Vec<Segment>) -> &dyn DigitBuilder<Segment> {
        self.segments = segments.clone();
        self
    }

    fn build(&self) -> Digit {
        let mut hash_set = HashSet::new();
        for segment in &self.segments {
            hash_set.insert(segment.clone());
        }
        Digit(hash_set)
    }
}

fn main() {
    let unique_signal_count_list = [2, 3, 4, 7];
    let input: Vec<Entry> = INPUT.split('\n').into_iter().map(parse_entry).collect();

    let res = input.iter().fold(0, |acc, entry| {
        let mut template_segments: Vec<Segment> = find_segments(entry);

        let output_digits: Vec<Digit> = entry
            .1
            .iter()
            .map(|chars| {
                let segments = chars
                    .iter()
                    .map(|c| Segment::from_ref(&template_segments, c))
                    .collect::<Vec<Segment>>();

                Digit::from_segment().add_all(&segments).build()
            })
            .collect();

        let final_output = output_digits
            .iter()
            .map(|digit| {
                digit
                    .0
                    .iter()
                    .map(|s| s.position.clone())
                    .collect::<Vec<Position>>()
            })
            .map(|positions| find_value(positions))
            .enumerate()
            .fold(0, |acc, (idx, value)| {
                let base: u32 = match idx {
                    0 => 1000,
                    1 => 100,
                    2 => 10,
                    _ => 1,
                };
                acc + (base * value)
            });

        println!("final output {}", final_output);
        acc + final_output
    });

    println!("RES = {}", res);
}

fn parse_digits(digits_str: &str) -> Vec<Vec<Signal>> {
    let digits_iter = digits_str.trim().split(' ').into_iter();

    digits_iter.map(|digits| digits.chars().collect()).collect()
}

fn parse_entry(entry_str: &str) -> Entry {
    let mut entry_iter = entry_str.split('|').into_iter();
    let digits_str = entry_iter.next().unwrap();
    let output_str = entry_iter.next().unwrap();

    Entry(parse_digits(digits_str), parse_digits(output_str))
}

fn find_segments_digits_by(digits: &Vec<Digit>, count: usize) -> Vec<&Digit> {
    digits
        .iter()
        .filter(|&word| word.0.len() == count)
        .collect::<Vec<&Digit>>()
}

fn find_top(seven: &Digit, one: &Digit) -> Option<Segment> {
    let diff = seven
        .0
        .symmetric_difference(&one.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    Some(Segment {
        position: Position::Top,
        value: diff.first().unwrap().value,
    })
}

fn find_top_right(
    one: &Digit,
    unknown_1: &Digit,
    unknown_2: &Digit,
    unknown_3: &Digit,
) -> Option<Segment> {
    let diff1 = unknown_1
        .0
        .symmetric_difference(&unknown_2.0)
        .map(clone_segment)
        .collect::<HashSet<Segment>>();
    let diff2 = unknown_2
        .0
        .symmetric_difference(&unknown_3.0)
        .map(clone_segment)
        .collect::<HashSet<Segment>>();
    let diff3 = unknown_1
        .0
        .symmetric_difference(&unknown_3.0)
        .map(clone_segment)
        .collect::<HashSet<Segment>>();

    let all_diffs = diff1
        .union(&diff2)
        .map(clone_segment)
        .collect::<HashSet<Segment>>()
        .union(&diff3)
        .map(clone_segment)
        .collect::<HashSet<Segment>>();

    let top_right = all_diffs
        .intersection(&one.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>()
        .first()
        .unwrap()
        .clone();

    Some(Segment {
        position: Position::TopRight,
        value: top_right.value,
    })
}

fn find_bottom_left(
    six: &Digit,
    top_right: &Segment,
    five_segment_digits: &Vec<&Digit>,
) -> Option<Segment> {
    let binding = five_segment_digits
        .iter()
        .filter(|&digit| !digit.has_segment(top_right))
        .map(|d| d.clone())
        .collect::<Vec<&Digit>>();
    let five = binding.first();

    if let Some(&digit) = five {
        let binding = six
            .0
            .symmetric_difference(&digit.0)
            .map(clone_segment)
            .collect::<Vec<Segment>>();

        let bottom_left = binding.first();

        Some(Segment {
            position: Position::BottomLeft,
            value: bottom_left.unwrap().value,
        })
    } else {
        None
    }
}

fn clone_segment(segment: &Segment) -> Segment {
    segment.clone()
}

fn find_five<'a>(five_segment_digits: &'a Vec<&Digit>, top_right: &Segment) -> &'a Digit {
    let binding = five_segment_digits
        .iter()
        .filter(|&digit| !digit.has_segment(top_right))
        .map(|d| *d)
        .collect::<Vec<&Digit>>();
    binding.first().unwrap()
}

fn find_two<'a>(five_segment_digits: &'a Vec<&Digit>, bottom_left: &Segment) -> &'a Digit {
    let binding = five_segment_digits
        .iter()
        .filter(|&digit| digit.has_segment(bottom_left))
        .map(|d| *d)
        .collect::<Vec<&Digit>>();
    binding.first().unwrap()
}

fn find_three<'a>(five_segment_digits: &'a Vec<&Digit>, two: &Digit, five: &Digit) -> &'a Digit {
    let binding = five_segment_digits
        .iter()
        .filter(|&&digit| digit.ne(two) && digit.ne(five))
        .map(|d| *d)
        .collect::<Vec<&Digit>>();
    binding.first().unwrap()
}

fn find_bottom_right(two: &Digit, three: &Digit, bottom_left: &Segment) -> Option<Segment> {
    let diff = two
        .0
        .symmetric_difference(&three.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let binding = diff
        .iter()
        .filter(|&segment| segment.value != bottom_left.value)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let bottom_right = binding.first().unwrap();

    Some(Segment {
        position: Position::BottomRight,
        value: bottom_right.value,
    })
}

fn find_zero<'a>(
    six_segment_digits: &'a Vec<&Digit>,
    bottom_left: &Segment,
    top_right: &Segment,
) -> &'a Digit {
    let binding = six_segment_digits
        .iter()
        .filter(|&digit| digit.has_segment(bottom_left) && digit.has_segment(top_right))
        .map(|d| *d)
        .collect::<Vec<&Digit>>();
    binding.first().unwrap()
}

fn find_center(zero: &Digit, eight: &Digit) -> Option<Segment> {
    let diff = zero
        .0
        .symmetric_difference(&eight.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let center = diff.first().unwrap();

    Some(Segment {
        position: Position::Center,
        value: center.value,
    })
}

fn find_top_left(three: &Digit, eight: &Digit, bottom_left: &Segment) -> Option<Segment> {
    let diff = three
        .0
        .symmetric_difference(&eight.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let binding = diff
        .iter()
        .filter(|&segment| segment.value != bottom_left.value)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let top_left = binding.first().unwrap();

    Some(Segment {
        position: Position::TopLeft,
        value: top_left.value,
    })
}

fn find_bottom(
    four: &Digit,
    eight: &Digit,
    top: &Segment,
    bottom_left: &Segment,
) -> Option<Segment> {
    let diff = four
        .0
        .symmetric_difference(&eight.0)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let binding = diff
        .iter()
        .filter(|&segment| segment.value != bottom_left.value && segment.value != top.value)
        .map(clone_segment)
        .collect::<Vec<Segment>>();

    let bottom = binding.first().unwrap();

    Some(Segment {
        position: Position::Bottom,
        value: bottom.value,
    })
}

fn print(digits: &Vec<&Digit>) {
    for &d in digits {
        println!("{}", d)
    }
}

fn print_seg(segments: &Vec<Segment>) {
    print!("\n");
    for s in segments {
        println!("{:?}:{}", s.position, s.value)
    }
    println!("\n")
}

fn find_segments(entry: &Entry) -> Vec<Segment> {
    let mut template_digit: Vec<Segment> = Vec::new();

    let digits: Vec<Digit> = entry
        .0
        .iter()
        .map(|c| Digit::from_char().add_all(c).build())
        .collect();

    let one_binding = digits
        .iter()
        .filter(|&d| d.0.len() == 2)
        .collect::<Vec<&Digit>>();
    let one = one_binding.first().unwrap();
    println!("one  : {}", one);

    let seven_binding = digits
        .iter()
        .filter(|&d| d.0.len() == 3)
        .collect::<Vec<&Digit>>();
    let seven = seven_binding.first().unwrap();
    println!("seven: {}", seven);

    let four = digits
        .iter()
        .filter(|&d| d.0.len() == 4)
        .collect::<Vec<&Digit>>();
    let four = four.first().unwrap();
    println!("four : {}", four);
    let eight = digits
        .iter()
        .filter(|&d| d.0.len() == 7)
        .collect::<Vec<&Digit>>();
    let eight = eight.first().unwrap();
    println!("eight: {}", eight);

    // find Top
    let top = find_top(&seven, &one).unwrap();
    template_digit.push(top);

    // find TopRight
    let six_segment_digits = find_segments_digits_by(&digits, 6);
    let first = six_segment_digits[0];
    let second = six_segment_digits[1];
    let third = six_segment_digits[2];

    let top_right = find_top_right(&one, first, second, third).unwrap();

    template_digit.push(top_right);

    let six_binding = six_segment_digits
        .iter()
        .filter(|&digit| !digit.has_segment(&top_right))
        .map(|d| d.clone())
        .collect::<Vec<&Digit>>();
    let six = *six_binding.first().unwrap();
    println!("six  : {}", six);

    let five_segment_digits: Vec<&Digit> = find_segments_digits_by(&digits, 5);
    let bottom_left = find_bottom_left(&six, &top_right, &five_segment_digits).unwrap();

    template_digit.push(bottom_left);

    let six_segment_digits = find_segments_digits_by(&digits, 6);
    let zero = find_zero(&six_segment_digits, &bottom_left, &top_right);
    println!("zero : {}", zero);

    let center = find_center(zero, eight).unwrap();
    template_digit.push(center);

    let five = find_five(&five_segment_digits, &top_right);
    println!("five : {}", five);
    let two = find_two(&five_segment_digits, &bottom_left);
    println!("two  : {}", two);
    let three = find_three(&five_segment_digits, two, five);
    println!("three: {}", three);

    let bottom_right = find_bottom_right(two, three, &bottom_left).unwrap();
    template_digit.push(bottom_right);

    let top_left = find_top_left(three, eight, &bottom_left).unwrap();
    template_digit.push(top_left);

    let bottom = find_bottom(four, eight, &top, &bottom_left).unwrap();
    template_digit.push(bottom);

    template_digit
}

fn find_value(segments: Vec<Position>) -> u32 {
    if segments.len() == 4 {
        4
    } else if segments.len() == 3 {
        7
    } else if segments.len() == 7 {
        8
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopLeft)
        && segments.contains(&&Position::TopRight)
        && segments.contains(&&Position::Center)
        && segments.contains(&&Position::BottomRight)
        && segments.contains(&&Position::Bottom)
    {
        9
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopLeft)
        && segments.contains(&&Position::Center)
        && segments.contains(&&Position::BottomLeft)
        && segments.contains(&&Position::BottomRight)
        && segments.contains(&&Position::Bottom)
    {
        6
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopLeft)
        && segments.contains(&&Position::TopRight)
        && segments.contains(&&Position::BottomLeft)
        && segments.contains(&&Position::BottomRight)
        && segments.contains(&&Position::Bottom)
    {
        0
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopRight)
        && segments.contains(&&Position::Center)
        && segments.contains(&&Position::BottomLeft)
        && segments.contains(&&Position::Bottom)
    {
        2
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopRight)
        && segments.contains(&&Position::Center)
        && segments.contains(&&Position::BottomRight)
        && segments.contains(&&Position::Bottom)
    {
        3
    } else if segments.contains(&&Position::Top)
        && segments.contains(&&Position::TopLeft)
        && segments.contains(&&Position::Center)
        && segments.contains(&&Position::BottomRight)
        && segments.contains(&&Position::Bottom)
    {
        5
    } else {
        1
    }
}
