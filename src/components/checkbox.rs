use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props<'a> {
    label: String,
    id: &'a str,
    value: &'a Signal<Option<String>>,
}

#[component]
pub fn Checkbox<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! { cx,
      label {
          (props.label)
          input(id=props.id, type="checkbox", on:click= |_| {
            match &*props.value.get() {
              None => props.value.set(Some(props.id.to_owned())),
              Some(_) => props.value.set(None)
            }


          }, checked=match &*props.value.get() {
            None => false,
            Some(_) =>true
          })
        }
    }
}
