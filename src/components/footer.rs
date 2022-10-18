use sycamore::prelude::*;

#[component]
pub fn Footer<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
      footer(class="footer") {
        p {"Made with ❤️ from 🇺🇦 by " a(class="footer__link", href="https://twitter.com/Mirus_ua") {"Mirus"}}
        p {"Try to build a WASM app with "
        a(href="https://github.com/sycamore-rs/sycamore", class="footer__link") {"sycamore"}
        " by yourself"}
        p {"Check it out on " a(class="footer__link", href="https://github.com/Miruzz/wassword-generator") {"GitHub"}}
      }
    }
}
