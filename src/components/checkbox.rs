use sycamore::{
    prelude::*,
    rt::{Event, JsCast},
};
use web_sys::HtmlInputElement;

#[derive(Prop)]
pub struct Props<'a> {
    label: String,
    id: String,
    value: &'a Signal<Option<String>>,
}

#[component]
pub fn Checkbox<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    let checked = create_signal(cx, true);

    view! { cx,
      label {
          (props.label)
          input(id=props.id.clone(), type="checkbox", on:click= |e: Event| {
            if *checked.get()  {
              props.value.set(None);
              checked.set(false);
            } else {
              match e.target() {
                None => (),
                Some(target) => props.value.set(Some(target.unchecked_into::<HtmlInputElement>().id()))
              }
              checked.set(true)
            }
          }, checked=*checked.get())
        }
    }
}
