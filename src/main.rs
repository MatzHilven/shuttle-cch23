use actix_web::web::{Data, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod days;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
    local_uri = "postgres://postgres:postgres@localhost:21533/postgres"
    )] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(days::configure);
        cfg.app_data(Data::new(pool.clone()));
    };

    Ok(config.into())
}
