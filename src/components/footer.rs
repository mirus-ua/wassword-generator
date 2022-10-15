use sycamore::prelude::*;

#[component]
pub fn Footer<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
      footer {
        p {"Made with ❤️ from 🇺🇦"}
        p {"Try to build a WASM app with "
        a(href="https://github.com/sycamore-rs/sycamore", class="footer__link") {"sycamore"}
        " by yourself"}
      }
    }
}
