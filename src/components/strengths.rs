use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props<'a> {
    value: &'a ReadSignal<String>,
}

#[component]
pub fn Strength<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! {
      cx,
      div(class="form__strength-container") {
        "STRENGTH"
        div(class="form__bars-container") {
          "bars"
        }
      }
    }
}
