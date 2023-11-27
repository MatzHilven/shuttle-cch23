use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/-1/error")]
async fn error() -> impl Responder {
    HttpResponse::InternalServerError().body("Error!")
}
