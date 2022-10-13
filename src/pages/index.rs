use sycamore::prelude::*;

use crate::components::{form::Form, header::Header, result::Result};

#[component]
pub fn Index<T: Html>(cx: Scope) -> View<T> {
    let result = create_signal(cx, "P4$w0rD!".to_owned());

    view! { cx,
      main(cla7ss="index-page") {
        Header
        Result(result=result)
        Form(result=result)
      }
    }
}
