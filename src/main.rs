pub mod components;
pub mod pages;
pub mod utils;

use sycamore::prelude::*;

use crate::pages::index::Index;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Index
        }
    });
}
