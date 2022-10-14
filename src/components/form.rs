use sycamore::prelude::*;

use super::{checkbox::Checkbox, range::Range};
use crate::utils::generator::string_generator;

#[derive(strum_macros::Display)]
pub enum CharTypes {
    Upper,
    Lower,
    Number,
    Symbol,
}

#[derive(Prop)]
pub struct Props<'a> {
    result: &'a Signal<String>,
}

#[component]
pub fn Form<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    let range = create_signal(cx, String::from("8"));
    let isUpper = create_signal(cx, Some(&CharTypes::Upper));
    let isLower = create_signal(cx, Some(&CharTypes::Lower));
    let isNumber = create_signal(cx, Some(&CharTypes::Number));
    let isSymbols = create_signal(cx, Some(&CharTypes::Symbol));

    let handle_click = |_| {
        let rng = range
            .get()
            .to_owned()
            .parse::<usize>()
            .expect("Range value is incorrect");
        let random_password = string_generator(rng, [isUpper, isLower, isNumber, isSymbols]);

        props.result.set(random_password);
    };

    let is_button_disabled = || {
        let rng = range.get().to_owned().parse::<i32>().unwrap();
        if rng == 0 {
            true
        } else {
            false
        }
    };

    view! { cx,
      form(class="form__container paper") {
        Range(amount_of_numbers=range)
        Checkbox(label="Include Uppercase Letters".to_owned(), id=&CharTypes::Upper, value=isUpper)
        Checkbox(label="Include Lowercase Letters".to_owned(), id=&CharTypes::Lower, value=isLower)
        Checkbox(label="Include Numbers".to_owned(), id=&CharTypes::Number, value=isNumber)
        Checkbox(label="Include Uppercase Letters".to_owned(), id=&CharTypes::Symbol, value=isSymbols)
        button(type="button", on:click=handle_click, disabled=is_button_disabled()) {"Generate"}
      }
    }
}
