use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props<'a> {
    amount_of_numbers: &'a Signal<String>,
}

#[component]
pub fn Range<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! {
      cx,
      p(class="form__range-text-container") {
        span { "Characters length" }
        span { (props.amount_of_numbers.get()) }
      }
      input(class="form__range", type="range", min=0, max=16, value=props.amount_of_numbers.get(), bind:value=props.amount_of_numbers)
    }
}
