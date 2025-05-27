use maud::{Markup, html};

use crate::templates::page;

pub async fn index() -> Markup {
    page(
        "Principal",
        html! {
            h1 { "LogBack" }
        },
    )
}
