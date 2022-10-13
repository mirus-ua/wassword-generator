use sycamore::prelude::*;

#[component]
pub fn Header<T: Html> (cx: Scope) -> View<T> {
  view! { cx,
    h2(class="index-page__header") {"Password Generator"}
  }
}