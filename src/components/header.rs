use sycamore::prelude::*;

#[component]
pub fn Header<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
      header(class="header") {
        h2(class="header__content") {
          span(class="header__logo-highlight") { "WA" }
          "ssword Generator"
        }
      }
    }
}
