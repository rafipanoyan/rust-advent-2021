fn day_3_part_2() {
    let input: Vec<String> = INPUT.split('\n').map(|s| s.trim().to_owned()).collect();

    let oxygen_word = get_oxygen_generator_rating(&input, 0, &|digit_count: DigitCount| -> char {
        if digit_count.0 > digit_count.1 {
            '0'
        } else {
            '1'
        }
    }).unwrap();
    let oxygen = u16::from_str_radix(&oxygen_word, 2).unwrap();
    println!("Oxygen Generator Rating: {}", oxygen);

    let co2_word = get_oxygen_generator_rating(&input, 0, &|digit_count: DigitCount| -> char {
        if digit_count.0 <= digit_count.1 {
            '0'
        } else {
            '1'
        }
    }).unwrap();
    let co2 = u16::from_str_radix(&co2_word, 2).unwrap();
    println!("CO2 Scubber rating: {}", co2);

    let life_support_rating: f64 = <u16 as Into<f64>>::into(oxygen) * <u16 as Into<f64>>::into(co2);
    println!("Life Support rating: {}", life_support_rating);
}

struct DigitCount(u32, u32);

fn get_oxygen_generator_rating(
    input: &Vec<String>,
    position: usize,
    mapper: &dyn Fn(DigitCount) -> char,
) -> Option<String> {
    if input.len() == 1 {
        return match input.first() {
            Some(value) => Option::Some(value.to_owned()),
            _ => Option::None,
        };
    }

    let count = get_digit_count(input, position);
    let discriminant = mapper(count);
    let new_input = input
        .iter()
        .filter(|&word| match word.chars().nth(position) {
            Some(bit) => bit == discriminant,
            _ => false,
        })
        .map(|word| word.to_owned())
        .collect::<Vec<String>>();

    println!("Position {}: {} element left.", position, new_input.len());

    return get_oxygen_generator_rating(&new_input, position + 1, mapper);
}

fn get_digit_count(input: &Vec<String>, position: usize) -> DigitCount {
    input
        .iter()
        .map(|word| -> u32 { get_digit_at(word, position) })
        .fold(DigitCount(0, 0), |acc, d| match d {
            0 => DigitCount(acc.0 + 1, acc.1),
            1 => DigitCount(acc.0, acc.1 + 1),
            _ => acc,
        })
}

fn get_digit_at(word: &str, position: usize) -> u32 {
    let char_at = word.chars().nth(position);
    char_at.unwrap_or('0').to_digit(10).unwrap_or(0)
}
