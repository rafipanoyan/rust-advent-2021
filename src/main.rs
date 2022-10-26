use data::data::{INPUT, INPUT_DEBUG};

mod data;

type Signal = char;

#[derive(Debug)]
struct Entry(Vec<Vec<Signal>>, Vec<Vec<Signal>>);

fn main() {
    let unique_signal_count_list = [
        2, 3, 4, 7
    ];
    let input: Vec<Entry> = INPUT.split('\n')
        .into_iter()
        .map(parse_entry).collect();

    let res = input.iter().fold(0, |acc, entry| {
        acc + entry.1.iter().fold(0, |acc, outputs| {
            if unique_signal_count_list.contains(&outputs.len()) {
                acc + 1
            } else {
                acc
            }
        })
    });

    println!("1, 4, 7 and 8 appear {} times",  res);
}

fn parse_digits(digits_str: &str) -> Vec<Vec<Signal>> {
    let mut digits_iter = digits_str.trim().split(' ').into_iter();

    digits_iter.map(|digits| {
        digits.chars().collect()
    }).collect()
}

fn parse_entry(entry_str: &str) -> Entry {
    let mut entry_iter = entry_str.split('|').into_iter();
    let digits_str = entry_iter.next().unwrap();
    let output_str = entry_iter.next().unwrap();

    Entry(parse_digits(digits_str), parse_digits(output_str))
}