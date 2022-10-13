use rand::Rng;
use sycamore::reactive::Signal;

fn get_random_upper() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(65..90)
}

fn get_random_lower() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(97..122)
}

fn get_random_number() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(48..57)
}

fn get_random_symbol() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(42..47)
}

pub fn string_generator(
    range: usize,
    is_upper: &Signal<String>,
    is_lower: &Signal<String>,
    is_number: &Signal<String>,
    is_symbol: &Signal<String>,
) -> String {
    let mut rng = rand::thread_rng();
    let mut generators: Vec<&dyn Fn() -> u8> = vec![];

    for char_type in [is_upper, is_lower, is_number, is_symbol] {
        match char_type.get().as_str() {
            "upper" => generators.push(&get_random_upper),
            "lower" => generators.push(&get_random_lower),
            "number" => generators.push(&get_random_number),
            "symbol" => generators.push(&get_random_symbol),
            _ => (),
        }
    }

    let mut chars_range: Vec<u8> = vec![0; range];

    for el in &mut chars_range {
        let rand_generator_index = rng.gen_range(0..generators.len());
        *el = generators[rand_generator_index]();
    }

    String::from_utf8(chars_range).unwrap()
}
