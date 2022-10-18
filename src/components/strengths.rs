use sycamore::prelude::*;

use crate::utils::check_strength::check_strength;

use super::bar::Bar;

const BARS_AMOUNT: usize = 4;

#[derive(Prop)]
pub struct Props<'a> {
    value: &'a ReadSignal<String>,
}

#[component]
pub fn Strength<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    let bars = create_signal(cx, vec![0; BARS_AMOUNT]);
    // let strength = create_selector(cx, || check_strength(&*props.value.get()));
    let bars_with_strength = bars.map(cx, |item| {
        let strength = check_strength(props.value);
        item.iter()
            .enumerate()
            .map(|(i, _itm)| {
                if i < usize::from(strength) {
                    strength
                } else {
                    0u8
                }
            })
            .collect::<Vec<u8>>()
    });

    view! {
      cx,
      div(class="form__strength-container") {
        "STRENGTH"
        div(class="form__bars-container") {
            Keyed(
              iterable=bars_with_strength,
              view=move |cx, x| view! { cx,
                Bar(level=x)
              },
              key=|x| *x,
          )
        }
      }
    }
}
