use rand::Rng;
use sycamore::reactive::Signal;

use crate::components::form::CharTypes;

fn get_random_char_number_from_range(from: u8, to: u8) -> impl Fn() -> u8 {
    move || {
        let mut rng = rand::thread_rng();
        rng.gen_range(from..to)
    }
}
pub fn string_generator<'a>(
    range: usize,
    char_options: [&Signal<Option<&'a CharTypes>>; 4],
) -> String {
    let mut rng = rand::thread_rng();
    let mut generators: Vec<&dyn Fn() -> u8> = vec![];
    let gen_upper = get_random_char_number_from_range(65, 90);
    let gen_lower = get_random_char_number_from_range(97, 122);
    let gen_number = get_random_char_number_from_range(48, 57);
    let gen_symbol = get_random_char_number_from_range(42, 47);

    for char_option in char_options {
        match &*char_option.get() {
            None => (),
            Some(char_type) => match char_type {
                CharTypes::Upper => generators.push(&gen_upper),
                CharTypes::Lower => generators.push(&gen_lower),
                CharTypes::Number => generators.push(&gen_number),
                CharTypes::Symbol => generators.push(&gen_symbol),
            },
        }
    }

    let mut chars_range: Vec<u8> = vec![];

    loop {
        if chars_range.len() == range {
            break;
        }

        let rand_generator_index = rng.gen_range(0..generators.len());
        let rand_res = generators[rand_generator_index]();

        if !chars_range.contains(&rand_res) {
            chars_range.push(rand_res);
        }
    }

    String::from_utf8(chars_range).unwrap()
}
