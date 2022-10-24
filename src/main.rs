use std::collections::HashMap;

use data::data::{INPUT, INPUT_DEBUG};

mod data;

fn main() {
    let mut lanterfishes: HashMap<i8, i64> = HashMap::new();

    INPUT
        .split(',')
        .into_iter()
        .map(|timer| timer.parse::<i8>().unwrap())
        .for_each(|timer| {
            let count = lanterfishes.entry(timer).or_insert(0);
            *count += 1;
        });

    let days = 256;

    for day in 0..days {
        let mut diff: HashMap<i8, i64> = HashMap::new();

        lanterfishes.iter()
            .for_each(|(timer, count)| {
                if *timer == 0 {
                    let decrement_depleted_fish = diff.entry(*timer).or_insert(0);
                    *decrement_depleted_fish -= count;

                    let increment_reseted_fish = diff.entry(6).or_insert(0);
                    *increment_reseted_fish += count;

                    let increment_new_fish = diff.entry(8).or_insert(0);
                    *increment_new_fish += count;
                } else {
                    let decrement_previous_timer = diff.entry(*timer).or_insert(0);
                    *decrement_previous_timer -= count;

                    let increment_new_timer = diff.entry(*timer - 1).or_insert(0);
                    *increment_new_timer += count;
                }
            });

        diff.iter().for_each(|(timer, count)| {
            let fishes_count = lanterfishes.entry(*timer).or_insert(0);
            *fishes_count += count;
        });

        println!("{:?}", lanterfishes);
    }

    let final_count = lanterfishes.iter().fold(0, |acc, (key, value)| {
        acc + value
    });

    println!("After {} days: {} fishes", days, final_count);
}