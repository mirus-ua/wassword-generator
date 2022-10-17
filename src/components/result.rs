use sycamore::prelude::*;
use web_sys::window;

#[derive(Prop)]
pub struct Props<'a> {
    result: &'a ReadSignal<String>,
}

#[component]
pub fn Result<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    let is_empty = create_selector(cx, || props.result.get().is_empty());

    view! {
      cx,
      div(class="paper form__result") {
        (if *is_empty.get() {
          view! { cx, p(class="form__result-pass form__password-placeholder") { "P4$w0rD!" } }
        } else {
          view! { cx, p(class="form__result-pass") { (props.result.get()) } }
        })
        button(class=if *is_empty.get() {"form__result-button"} else {"form__result-button form__result-button-active"}, type="button", on:click=|_|{
          let _promise = window().unwrap().navigator().clipboard().unwrap().write_text(&props.result.get());
        }) { "📋" }
      }
    }
}
