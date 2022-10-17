use sycamore::prelude::*;

use crate::components::{footer::Footer, form::Form, header::Header, result::Result};

#[component]
pub fn Index<T: Html>(cx: Scope) -> View<T> {
    let result = create_signal(cx, String::from(""));

    view! { cx,
      main(class="index-page") {
        Header
        Result(result=result)
        Form(result=result)
        Footer
      }
    }
}
