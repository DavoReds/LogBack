use maud::{DOCTYPE, Markup, html};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charser="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta name="color-scheme" content="light dark";

                title { (title) " | LogBack" }
                meta name="description" content="Aplicación para administrar listas de pendientes";
            }
            body {
                main .container {
                    (content)
                }
            }
        }
    }
}
