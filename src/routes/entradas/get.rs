use actix_web::web;
use maud::{Markup, html};
use sqlx::PgPool;

use crate::templates::page;

pub async fn get_entradas(pool: web::Data<PgPool>) -> Markup {
    let entradas = match sqlx::query!(
        r"SELECT
        e.id, e.nombre, t.nombre as tipo, t.color AS color_tipo,
        s.nombre as estado, s.color AS color_estado
        FROM entradas e
        JOIN tipos t ON (e.tipo = t.id)
        JOIN estados s ON (e.estado = s.id)
        ORDER BY t.nombre"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(entradas) => entradas,
        Err(e) => {
            log::error!("Error en la base de datos: {e}");

            return page(
                "Entradas",
                html! {
                    section #entradas {
                        h1 { "Entradas" }
                        p { "Error trayendo referencias. Por favor intente nuevamente." }
                    }
                },
            );
        }
    };

    page(
        "Entradas",
        html! {
            section #entradas
             {
                h1 { "Entradas" }

                table .striped {
                    thead {
                        tr {
                            th scope="col" { "Nombre" }
                            th scope="col" { "Tipo" }
                            th scope="col" { "Estado" }
                            th scope="col" { "Controles" }
                        }
                    }
                    tbody {
                        @for entrada in entradas {
                            tr {
                                td { (entrada.nombre) }
                                td style={"color:" (entrada.color_tipo)} { (entrada.tipo) }
                                td style={"color:" (entrada.color_estado)} { (entrada.estado) }
                                td {
                                    div.grid {
                                        button .contrast {
                                            img width="16" alt="Editar" src="/public/img/lapiz.svg";
                                        }
                                        button {
                                            img width="16" alt="Eliminar" src="/public/img/cross.svg";
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}
