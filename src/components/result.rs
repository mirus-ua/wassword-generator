use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props<'a> {
    result: &'a ReadSignal<String>,
}

#[component]
pub fn Result<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! {
      cx,
      div {
        p {(props.result.get())}
        button { "copy" }
      }
    }
}
