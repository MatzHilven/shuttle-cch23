use actix_web::{post, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {}


#[post("/22")]
async fn part_1(string: String) -> impl Responder {
    string
}

#[post("/22")]
async fn part_2(string: String) -> impl Responder {
    string
}
