mod get;
mod post;

use compact_str::CompactString;
pub use get::get_nuevo_usuario;
pub use post::post_usuario;

use crate::templates::page_no_header;
use maud::{Markup, html};

#[derive(Debug)]
pub enum ErrorNuevoUsuario {
    Nombre(CompactString),
    Clave(CompactString),
}

impl std::fmt::Display for ErrorNuevoUsuario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nombre(err) | Self::Clave(err) => write!(f, "{err}"),
        }
    }
}

pub fn formulario_nuevo_usuario(error: Option<ErrorNuevoUsuario>) -> Markup {
    page_no_header(
        "Nuevo Usuario",
        html! {
            h1 { "Nuevo Usuario" }

            form action="/usuarios" method="POST" {
                fieldset {
                    label {
                        "Nombre"
                        input
                            type="text"
                            maxlength="100"
                            name="nombre"
                            autocomplete="username"
                            required
                            aria-describedby="nombre-helper"
                            aria-invalid=[{
                               if matches!(error, Some(ErrorNuevoUsuario::Nombre(_))) {
                                    Some("true")
                                } else {
                                    None
                                }
                            }];

                        small #nombre-helper {
                            @if let Some(ErrorNuevoUsuario::Nombre(ref err)) = error {
                                (err)
                            }
                        }
                    }
                    label {
                        "Contraseña"
                        input
                            type="password"
                            name="clave"
                            autocomplete="new-password"
                            required
                            aria-invalid=[{
                               if matches!(error, Some(ErrorNuevoUsuario::Clave(_))) {
                                    Some("true")
                                } else {
                                    None
                                }
                            }];
                    }
                    label {
                        "Confirmar contraseña"
                        input
                            type="password"
                            name="confirmar"
                            autocomplete="off"
                            required
                            aria-describedby="clave-helper"
                            aria-invalid=[{
                               if matches!(error, Some(ErrorNuevoUsuario::Clave(_))) {
                                    Some("true")
                                } else {
                                    None
                                }
                            }];

                        small #clave-helper {
                            @if let Some(ErrorNuevoUsuario::Clave(ref err)) = error {
                                (err)
                            }
                        }
                    }
                }
                input type="submit" value="Crear";
            }
        },
    )
}
