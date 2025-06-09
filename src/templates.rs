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

                script defer src="/public/js/htmx.min.js" {}
                link rel="stylesheet" href="/public/css/pico.pink.min.css";

                title { (title) " | LogBack" }
                meta name="description" content="Aplicación para administrar listas de pendientes";
            }
            body {
                div .container hx-boost="true" {
                    header {
                        nav {
                            ul {
                                li {
                                    a href="/" { strong { "LogBack" } }
                                }
                            }
                            ul {
                                li {
                                    a .contrast href="/entradas" { "Entradas" }
                                }
                                li {
                                }
                            }
                        }
                    }
                    main {
                        (content)
                    }
                }
            }
        }
    }
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn page_no_header(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charser="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta name="color-scheme" content="light dark";

                script defer src="/public/js/htmx.min.js" {}
                link rel="stylesheet" href="/public/css/pico.pink.min.css";

                title { (title) " | LogBack" }
                meta name="description" content="Aplicación para administrar listas de pendientes";
            }
            body {
                main .container hx-boost="true" {
                    (content)
                }
            }
        }
    }
}
