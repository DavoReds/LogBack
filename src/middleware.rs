use actix_web::{
    HttpResponse,
    body::{BoxBody, EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Redirige a la página de creación de usuarios si ninguno existe en la base
/// de datos.
pub async fn existen_usuarios(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<
    ServiceResponse<EitherBody<impl MessageBody, BoxBody>>,
    actix_web::Error,
> {
    let usuario_existe = req
        .app_data::<web::Data<AtomicBool>>()
        .expect("Estado debe existir");

    if usuario_existe.load(Ordering::Relaxed) {
        let response = next.call(req).await?;
        return Ok(response.map_body(|_, body| EitherBody::left(body)));
    }

    let response = req.into_response(
        HttpResponse::SeeOther()
            .insert_header(("Location", "/usuarios"))
            .finish(),
    );

    Ok(response.map_body(|_, body| EitherBody::right(body)))
}
