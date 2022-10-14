use sycamore::prelude::*;
// use web_sys::Clipboard;

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

          // let clp = Clipboard {};
          // Clipboard::write_text(&clp ,&props.result.get());
        }) { "copy" }
      }
    }
}
