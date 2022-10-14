use sycamore::prelude::*;
use web_sys::window;

#[derive(Prop)]
pub struct Props<'a> {
    result: &'a ReadSignal<String>,
}

#[component]
pub fn Result<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! {
      cx,
      div(class="paper") {
        p {(props.result.get())}
        button(type="button", on:click=|_|{
          let _promise = window().unwrap().navigator().clipboard().unwrap().write_text(&props.result.get());
        }) { "copy" }
      }
    }
}
