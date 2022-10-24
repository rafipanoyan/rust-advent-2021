use data::data::{INPUT, INPUT_DEBUG};

mod data;

#[derive(PartialEq, Ord, Eq)]
struct HorizontalPosition {
    position: i128,
    fuel: u128
}

impl HorizontalPosition {
    fn new(position: i128, fuel: u128) -> HorizontalPosition {
        HorizontalPosition { position, fuel }
    }
}

impl PartialOrd for HorizontalPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fuel.partial_cmp(&other.fuel)
    }
}

fn main() {

    let positions = INPUT
        .split(',')
        .into_iter()
        .map(|position| position.parse::<i128>().unwrap());
    
    let min_pos = positions.clone().min().unwrap();
    let max_pos = positions.clone().max().unwrap();

    println!("Min: {} | Max: {}", min_pos, max_pos);

    let mut possible_position: Vec<HorizontalPosition> = Vec::new();
    for possible_pos in min_pos..(max_pos + 1) {

        let fuel = positions.clone().into_iter().fold(0, |acc, p| {
            let diff = p.abs_diff(possible_pos);
            acc + step_cost(diff)
        });
        possible_position.push(HorizontalPosition::new(possible_pos, fuel));
    }

    // let min_fuel_pos = possible_position.iter().min().unwrap();
    let min_fuel_pos = possible_position.iter().fold(None, |acc, pos| {
        match acc {
            None => Some(pos),
            Some(current_min) => {
                if pos.fuel < current_min.fuel {
                    Some(pos)
                } else {
                    acc
                }
            }
        }
    }).unwrap();

    println!("Min fuel position: {}, fuel: {}", min_fuel_pos.position, min_fuel_pos.fuel);
}

fn step_cost(step: u128) -> u128 {
    match step {
        0 => 0,
        _ => step + step_cost(step - 1)
    }
}