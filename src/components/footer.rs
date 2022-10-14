use sycamore::prelude::*;

#[component]
pub fn Footer<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
      footer {
        p {"Made with ❤️ from 🇺🇦"}
        p {"Try to build with "
        a(href="https://github.com/sycamore-rs/sycamore") {"sycamore"}
        " by yourself"}
      }
    }
}