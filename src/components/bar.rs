use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props {
    level: u8,
}

#[component]
pub fn Bar<T: Html>(cx: Scope, props: Props) -> View<T> {
    let bar_fill = match props.level {
        0 => "0",
        1 => "form__strength-low",
        2 => "form__strength-mid",
        3 => "form__strength-good",
        4 => "form__strength-superb",
        _ => "",
    };

    let class = "form__strength-bar ".to_owned() + bar_fill;

    view! {
      cx,
      span(class=class) {}
    }
}
