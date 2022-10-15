use sycamore::prelude::*;

use super::form::CharTypes;

#[derive(Prop)]
pub struct Props<'a> {
    label: String,
    id: &'a CharTypes,
    value: &'a Signal<Option<&'a CharTypes>>,
}

#[component]
pub fn Checkbox<'a, T: Html>(cx: Scope<'a>, props: Props<'a>) -> View<T> {
    view! { cx,
      label(class="form__checkbox-label") {
          (props.label)
          input(id=props.id, type="checkbox", on:click= |_| {
            match &*props.value.get() {
              None => props.value.set(Some(props.id)),
              Some(_) => props.value.set(None)
            }

          },
          checked=match &*props.value.get() {
            None => false,
            Some(_) =>true
          })
        }
    }
}
