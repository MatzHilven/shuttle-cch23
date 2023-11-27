use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

mod days;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(days::zero::hello_world);
    };

    Ok(config.into())
}
