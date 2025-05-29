use actix_web::Responder;
use maud::html;

use crate::templates::page;

pub async fn get_entradas() -> impl Responder {
    page(
        "Entradas",
        html! {
            section #entradas {
                h1 { "Entradas" }
            }
        },
    )
}
