fn day_3_part_1() {
    let input: Vec<&str> = INPUT.split('\n').map(|s| s.trim()).collect();

    let input_lenght = input.first().unwrap().chars().count();

    let mut gamma_res: String = String::new();
    let mut epsilon_res: String = String::new();
    for position in 0..input_lenght {
        let most_bit = get_bit(&input, position, &|digit_count: DigitCount| {
            if digit_count.0 > digit_count.1 {
                0
            } else {
                1
            }
        });
        let most_bit = from_digit(most_bit.into(), 10).unwrap_or('0');
        gamma_res.push(most_bit);

        let least_bit = get_bit(&input, position, &|digit_count: DigitCount| {
            if digit_count.0 > digit_count.1 {
                1
            } else {
                0
            }
        });
        let least_bit = from_digit(least_bit.into(), 10).unwrap_or('0');
        epsilon_res.push(least_bit);
    }

    let gamma_decimal = isize::from_str_radix(&gamma_res, 2).unwrap();
    let epsilon_decimal = isize::from_str_radix(&epsilon_res, 2).unwrap();

    println!(
        "Power Consumption is : {}",
        gamma_decimal * epsilon_decimal
    );
}

struct DigitCount(u32, u32);

fn get_bit(input: &Vec<&str>, position: usize, mapper: &dyn Fn(DigitCount) -> u8) -> u8 {
     mapper(get_digit_count(input, position))
}

fn get_digit_count(input: &Vec<&str>, position: usize) -> DigitCount {
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