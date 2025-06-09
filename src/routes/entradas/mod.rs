mod delete;
mod get;
mod post;
mod put;

pub use delete::delete_entrada;
pub use get::get_entradas;
pub use post::post_entradas;
pub use put::{get_formulario_entrada, put_entrada};
